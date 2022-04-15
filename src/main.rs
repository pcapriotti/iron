mod game;
mod glyphs;
mod graphics;
mod layout;
mod tiles;

use game::{Direction, Game};
use glow::HasContext;
use glutin::event::{ElementState, Event, VirtualKeyCode};
use glutin::event_loop::ControlFlow;
use glyphs::Glyphs;
use layout::Layout;
use std::time::{Duration, Instant};
use tiles::Tiles;

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
        ctx.enable(glow::DEPTH_TEST);
        ctx
    };

    // init a game state
    let mut game = Game::new(4, 4);
    game.add_random_tile();

    let mut glyphs = Glyphs::new(&gl);
    let mut tiles = Tiles::new(&gl);

    let mut layout = Layout::compute(0, 0, game.width(), game.height());

    eloop.run(move |e, _target, cf| {
        *cf =
            ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(16));
        match e {
            Event::LoopDestroyed => {
                glyphs.cleanup(&gl);
                tiles.cleanup(&gl);
            }
            Event::RedrawRequested(_) => {
                unsafe {
                    gl.clear_color(0.148, 0.148, 0.148, 1.0);
                    gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
                    tiles.render(&gl);
                    glyphs.render(&gl);
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
                        tiles.resize(&gl, sz.width, sz.height);
                        tiles.update(&gl, &layout, &game);
                        glyphs.resize(&gl, sz.width, sz.height);
                        glyphs.update(&gl, &layout, &game);
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
                            if let Some(d) = dir {
                                game.step(d);
                                game.add_random_tile();
                                tiles.update(&gl, &layout, &game);
                                glyphs.update(&gl, &layout, &game);
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
