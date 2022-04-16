mod animation;
mod game;
mod glyphs;
mod graphics;
mod layout;
mod scene;
mod tiles;

use animation::{Animation, MoveAnimation};
use game::{Direction, Game};
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
    let mut anim: Option<MoveAnimation> = None;

    eloop.run(move |e, _target, cf| {
        *cf = ControlFlow::Wait;
        match e {
            Event::LoopDestroyed => {
                scene.cleanup(&gl);
            }
            Event::RedrawRequested(_) => {
                if let Some(MoveAnimation { animation, moves }) = &anim {
                    let t = animation.time().min(1.0);
                    let done = t >= 1.0;

                    if done {
                        game.add_random_tile();
                    }
                    scene.update(&gl, &layout, &game, moves, t);
                    win.window().request_redraw();

                    if done {
                        anim = None;
                    }
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
                        scene.update(&gl, &layout, &game, &[], 0.0);
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
                            if anim.is_some() {
                                return;
                            }
                            if let Some(d) = dir {
                                let moves = game.step(d);
                                if !moves.is_empty() {
                                    scene.update(
                                        &gl, &layout, &game, &moves, 0.0,
                                    );
                                    anim = Some(MoveAnimation {
                                        animation: Animation::new(
                                            Animation::DEFAULT_DURATION,
                                        ),
                                        moves: moves,
                                    });
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
