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
use glutin::event::{ElementState, Event, ModifiersState, VirtualKeyCode};
use glutin::event_loop::ControlFlow;
use layout::Layout;
use scene::Scene;
use std::rc::Rc;
use std::time::Duration;

const INITIAL_SIZE: (u32, u32) = (800, 600);

fn main() {
    // TODO: figure out why rendering on wayland is broken
    std::env::set_var("WAYLAND_DISPLAY", "");

    let eloop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Iron")
        .with_transparent(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(
            INITIAL_SIZE.0,
            INITIAL_SIZE.1,
        ));
    let wctx = glutin::ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(wb, &eloop)
        .unwrap();
    let win = unsafe { wctx.make_current().unwrap() };
    let gl = unsafe {
        let ctx = glow::Context::from_loader_function(|s| {
            win.get_proc_address(s) as *const _
        });
        ctx.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
        ctx.enable(glow::BLEND);
        ctx
    };
    let gl = Rc::new(gl);

    let config = get_config().unwrap_or(Config::default());
    let mut game = Game::new(config.width, config.height);
    game.add_random_tile();

    let mut scene = Scene::new(gl.clone(), &config);

    let mut layout = Layout::compute(
        INITIAL_SIZE.0,
        INITIAL_SIZE.1,
        game.width(),
        game.height(),
    );
    let mut anim: Option<Animation<Vec<Move>>> = None;
    let mut modifiers = ModifiersState::empty();

    win.window().request_redraw();

    eloop.run(move |e, _target, cf| {
        *cf = ControlFlow::Wait;
        match e {
            Event::LoopDestroyed => {}
            Event::RedrawRequested(_) => {
                unsafe {
                    render(&gl, &layout, &mut scene, &mut anim, &mut game, &win)
                };
            }
            Event::WindowEvent { event: ref e, .. } => {
                use glutin::event::WindowEvent;
                match e {
                    WindowEvent::Resized(sz) => {
                        win.resize(*sz);
                        unsafe {
                            gl.viewport(0, 0, sz.width as i32, sz.height as i32)
                        };

                        layout = Layout::compute(
                            sz.width,
                            sz.height,
                            game.width(),
                            game.height(),
                        );
                        scene.resize(sz.width, sz.height);
                    }
                    WindowEvent::CloseRequested => *cf = ControlFlow::Exit,
                    WindowEvent::ModifiersChanged(s) => {
                        modifiers = *s;
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(key) = input.virtual_keycode {
                            use VirtualKeyCode::*;
                            if input.state != ElementState::Pressed
                                || !modifiers.is_empty()
                            {
                                return;
                            }
                            if game.is_over() {
                                match key {
                                    Space | Return | N => {
                                        game = Game::new(
                                            game.width(),
                                            game.height(),
                                        );
                                        game.add_random_tile();
                                        anim = None;
                                        win.window().request_redraw();
                                    }
                                    _ => {}
                                };
                            } else {
                                let dir = match key {
                                    Escape | Q => {
                                        *cf = ControlFlow::Exit;
                                        None
                                    }
                                    Left | H => Some(Direction::W),
                                    Down | J => Some(Direction::S),
                                    Up | K => Some(Direction::N),
                                    Right | L => Some(Direction::E),
                                    _ => None,
                                };
                                // do not accept moves while another one is being animated
                                if let Some(_) = anim {
                                    return;
                                }
                                if let Some(d) = dir {
                                    let mut game2 = game.clone();
                                    let moves = game2.step(d);
                                    if !moves.is_empty() {
                                        game2.add_random_tile();
                                    }

                                    anim = Some(Animation::new(
                                        Duration::from_millis(
                                            config.animation_duration_ms,
                                        ),
                                        moves,
                                        game2,
                                    ));
                                    win.window().request_redraw();
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });
}

unsafe fn render(
    gl: &glow::Context,
    layout: &Layout,
    scene: &mut Scene,
    anim: &mut Option<Animation<Vec<Move>>>,
    game: &mut Game,
    win: &glutin::ContextWrapper<
        glutin::PossiblyCurrent,
        glutin::window::Window,
    >,
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
        win.window().request_redraw();
    } else {
        scene.update(&layout, &game, &Vec::new(), 1.0);
    }

    if cfg!(feature = "debug") {
        println!("{} us", (Instant::now() - start).as_micros());
    }
    win.swap_buffers().unwrap();
}

fn get_config() -> Option<Config> {
    let mut path = dirs::config_dir()?;
    path.push("iron");
    path.push("config.toml");

    let s = std::fs::read_to_string(path).ok()?;

    let config: Config = toml::from_str(&s).unwrap();

    Some(config)
}
