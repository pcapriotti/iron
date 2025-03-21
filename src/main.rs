mod animation;
mod config;
mod game;
mod glyphs;
mod graphics;
mod layout;
mod scene;
mod tiles;

use animation::Animation;
use config::Config;
use game::{Direction, Game, Move};
use glow::HasContext;
use glutin::{
    config::{ConfigTemplateBuilder, GlConfig},
    context::{ContextApi, ContextAttributesBuilder, NotCurrentGlContext, PossiblyCurrentContext},
    display::{GetGlDisplay, GlDisplay},
    prelude::GlSurface,
    surface::{Surface, SwapInterval, WindowSurface},
};
use glutin_winit::{DisplayBuilder, GlWindow};
use layout::Layout;
use scene::Scene;
use std::{num::NonZeroU32, rc::Rc, time::Duration};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::NamedKey,
    raw_window_handle::HasWindowHandle,
    window::{Window, WindowId},
};

const INITIAL_SIZE: (u32, u32) = (800, 600);

enum Key {
    Named(NamedKey),
    Character(char),
}

impl Key {
    pub fn from_event(event: &KeyEvent) -> Vec<Self> {
        let mut result = Vec::new();
        match &event.text {
            None => match event.logical_key {
                winit::keyboard::Key::Named(n) => {
                    result.push(Self::Named(n));
                }
                _ => (),
            },
            Some(t) => {
                result.extend(t.chars().map(Key::Character));
            }
        }
        result
    }
}

struct Display {
    gl: Rc<glow::Context>,
    gl_surface: Surface<WindowSurface>,
    gl_context: PossiblyCurrentContext,
    config: Config,
    layout: Layout,
    scene: Scene,
    animation: Option<Animation<Vec<Move>>>,
    game: Game,
    window: Window,
}

impl ApplicationHandler for Display {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => unsafe {
                render(
                    &self.gl,
                    &self.layout,
                    &mut self.scene,
                    &mut self.animation,
                    &mut self.game,
                    &mut self.window,
                );
                self.gl_surface.swap_buffers(&self.gl_context).unwrap();
            },
            WindowEvent::Resized(sz) => {
                unsafe { self.gl.viewport(0, 0, sz.width as i32, sz.height as i32) };

                self.layout =
                    Layout::compute(sz.width, sz.height, self.game.width(), self.game.height());
                self.scene.resize(sz.width, sz.height);
            }
            WindowEvent::KeyboardInput { ref event, .. } => {
                if event.state == ElementState::Pressed {
                    for key in Key::from_event(event) {
                        if self.game.is_over() {
                            match key {
                                Key::Character(' ' | '\r' | 'n') => {
                                    self.game = Game::new(self.game.width(), self.game.height());
                                    self.game.add_random_tile();
                                    self.animation = None;
                                    self.window.request_redraw();
                                }
                                _ => {}
                            };
                        } else {
                            let dir = match key {
                                Key::Character('\u{1b}' | 'q') => {
                                    event_loop.exit();
                                    None
                                }
                                Key::Named(NamedKey::ArrowLeft) | Key::Character('h') => {
                                    Some(Direction::W)
                                }
                                Key::Named(NamedKey::ArrowDown) | Key::Character('j') => {
                                    Some(Direction::S)
                                }
                                Key::Named(NamedKey::ArrowUp) | Key::Character('k') => {
                                    Some(Direction::N)
                                }
                                Key::Named(NamedKey::ArrowRight) | Key::Character('l') => {
                                    Some(Direction::E)
                                }
                                _ => None,
                            };
                            // do not accept moves while another one is being animated
                            if let Some(_) = self.animation {
                                return;
                            }
                            if let Some(d) = dir {
                                let mut game2 = self.game.clone();
                                let moves = game2.step(d);
                                if !moves.is_empty() {
                                    game2.add_random_tile();
                                }

                                self.animation = Some(Animation::new(
                                    Duration::from_millis(self.config.animation_duration_ms),
                                    moves,
                                    game2,
                                ));
                                self.window.request_redraw();
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let attrs = Window::default_attributes()
        .with_title("Iron")
        .with_transparent(false)
        .with_inner_size(winit::dpi::LogicalSize::new(INITIAL_SIZE.0, INITIAL_SIZE.1));
    let template = ConfigTemplateBuilder::new();
    let display_builder = DisplayBuilder::new().with_window_attributes(Some(attrs));
    let (window, gl_config) = display_builder
        .build(&event_loop, template, |configs| {
            configs
                .reduce(|accum, config| {
                    if config.num_samples() > accum.num_samples() {
                        config
                    } else {
                        accum
                    }
                })
                .unwrap()
        })
        .unwrap();
    let window = window.unwrap();
    let rwh = window.window_handle().unwrap();
    let gl_display = gl_config.display();
    let context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(glutin::context::Version {
            major: 4,
            minor: 1,
        })))
        .build(Some(rwh.into()));

    let not_current_gl_context = unsafe {
        gl_display
            .create_context(&gl_config, &context_attributes)
            .unwrap()
    };
    let attrs = window.build_surface_attributes(Default::default()).unwrap();
    let gl_surface = unsafe {
        gl_display
            .create_window_surface(&gl_config, &attrs)
            .unwrap()
    };

    let gl_context = not_current_gl_context.make_current(&gl_surface).unwrap();
    let gl = unsafe {
        let ctx = glow::Context::from_loader_function_cstr(|s| gl_display.get_proc_address(s));
        ctx.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
        ctx.enable(glow::BLEND);
        ctx
    };
    let gl = Rc::new(gl);

    gl_surface
        .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
        .unwrap();

    let config = get_config().unwrap_or(Config::default());
    let mut game = Game::new(config.width, config.height);
    game.add_random_tile();

    let scene = Scene::new(gl.clone(), &config);
    let layout = Layout::compute(INITIAL_SIZE.0, INITIAL_SIZE.1, game.width(), game.height());
    let animation: Option<Animation<Vec<Move>>> = None;

    let mut display = Display {
        gl,
        gl_surface,
        gl_context,
        config,
        layout,
        game,
        animation,
        scene,
        window,
    };
    event_loop.run_app(&mut display).unwrap();
}

unsafe fn render(
    gl: &glow::Context,
    layout: &Layout,
    scene: &mut Scene,
    anim: &mut Option<Animation<Vec<Move>>>,
    game: &mut Game,
    window: &mut Window,
) {
    use std::time::Instant;

    let start = Instant::now();

    gl.clear_color(0.148, 0.148, 0.148, 1.0);
    gl.clear(glow::COLOR_BUFFER_BIT);
    if let Some(a) = &anim {
        let t = a.time().min(1.0);
        if t >= 1.0 {
            let a = anim.take().unwrap();
            *game = a.result;
            scene.update(&layout, &game, &Vec::new(), 1.0);
        } else {
            scene.update(&layout, &game, &a.inner, t);
        }
        window.request_redraw();
    } else {
        scene.update(&layout, &game, &Vec::new(), 1.0);
    }

    if cfg!(feature = "debug") {
        println!("{} us", (Instant::now() - start).as_micros());
    }
}

fn get_config() -> Option<Config> {
    let mut path = dirs::config_dir()?;
    path.push("iron");
    path.push("config.toml");

    let s = std::fs::read_to_string(path).ok()?;

    let config: Config = toml::from_str(&s).unwrap();

    Some(config)
}
