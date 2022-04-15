mod game;
mod graphics;
mod tile;

use game::Game;
use glow::HasContext;
use glutin::event::{Event, VirtualKeyCode};
use glutin::event_loop::ControlFlow;
use std::time::{Duration, Instant};
use tile::Tile;

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

    let game = Game::new(4, 4);
    let mut tile = {
        let mut tile = Tile::new(&gl);
        tile.setup_grid(&gl, &game);
        Some(tile)
    };
    eloop.run(move |e, _target, cf| {
        *cf =
            ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(16));
        match e {
            Event::LoopDestroyed => {
                let mut tile = tile.take().unwrap();
                tile.cleanup(&gl);
            }
            Event::RedrawRequested(_) => {
                let tile = tile.as_mut().unwrap();
                draw_window(&gl, &tile);
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

                        if let Some(tile) = &mut tile {
                            // tile.set_scale(&gl, sz.width, sz.height, &size);
                            tile.resize(&gl, sz.width, sz.height);
                            tile.setup_grid(&gl, &game);
                        }
                    }
                    WindowEvent::CloseRequested => *cf = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(key) = input.virtual_keycode {
                            if key == VirtualKeyCode::Escape {
                                *cf = ControlFlow::Exit;
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

fn draw_window(gl: &glow::Context, tile: &Tile) {
    unsafe {
        gl.clear_color(0.148, 0.148, 0.148, 1.0);
        gl.clear(glow::COLOR_BUFFER_BIT);
    }

    tile.render(&gl);
}
