mod animation;
mod game;
mod glyphs;
mod graphics;
mod layout;
mod scene;
mod tiles;

use animation::Animation;
use game::{Direction, Game, Move};
use glow::HasContext;
use glutin::event::{ElementState, Event, VirtualKeyCode};
use glutin::event_loop::ControlFlow;
use layout::Layout;
use scene::Scene;

fn main() {
    let eloop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Iron")
        .with_transparent(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(800, 600));
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

    // init a game state
    let mut game = Game::new(4, 4);
    game.add_random_tile();

    let mut scene = Scene::new(&gl);

    let mut layout = Layout::compute(0, 0, game.width(), game.height());
    let mut anim: Option<Animation<Vec<Move>>> = None;

    eloop.run(move |e, _target, cf| {
        *cf = ControlFlow::Wait;
        match e {
            Event::LoopDestroyed => {
                scene.cleanup(&gl);
            }
            Event::RedrawRequested(_) => {
                if let Some(a) = &anim {
                    let t = a.time().min(1.0);
                    if t >= 1.0 {
                        let a = anim.take().unwrap();
                        game = a.result;
                        scene.update(&gl, &layout, &game, &Vec::new(), 1.0);
                    } else {
                        scene.update(&gl, &layout, &game, &a.inner, t);
                    }
                    win.window().request_redraw();
                }
                unsafe {
                    gl.clear_color(0.148, 0.148, 0.148, 1.0);
                    gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
                    scene.render(&gl);
                }
                win.swap_buffers().unwrap();
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
                        scene.resize(&gl, sz.width, sz.height);
                        scene.update(&gl, &layout, &game, &Vec::new(), 0.0);
                    }
                    WindowEvent::CloseRequested => *cf = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(key) = input.virtual_keycode {
                            use VirtualKeyCode::*;
                            if input.state != ElementState::Pressed {
                                return;
                            }
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
                                game2.add_random_tile();

                                anim = Some(Animation::new(
                                    animation::DEFAULT_DURATION,
                                    moves,
                                    game2,
                                ));
                                win.window().request_redraw();
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
