use glow::HasContext;
use glutin::event::{Event, VirtualKeyCode};
use glutin::event_loop::ControlFlow;
use std::time::{Duration, Instant};

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
    let gl =
        unsafe { glow::Context::from_loader_function(|s| win.get_proc_address(s) as *const _) };

    let program = unsafe {
        let vert = compile_shader_from_source(
            &gl,
            glow::VERTEX_SHADER,
            r#"
            #version 330 core
            layout (location = 0) in vec3 p;

            void main() {
                gl_Position = vec4(p.x, p.y, p.z, 1.0);
            }"#,
        );

        let frag = compile_shader_from_source(
            &gl,
            glow::FRAGMENT_SHADER,
            r#"
            #version 330 core

            out vec4 col;

            void main() {
                col = vec4(1.0, 0.0, 0.0, 1.0);
            }"#,
        );

        let program = gl.create_program().unwrap();
        gl.attach_shader(program, vert);
        gl.attach_shader(program, frag);
        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("link error: {}", gl.get_program_info_log(program));
        }

        gl.detach_shader(program, vert);
        gl.delete_shader(vert);

        gl.detach_shader(program, frag);
        gl.delete_shader(frag);
        program
    };

    let (vbo, vao) = unsafe {
        let vertices: [f32; 20] = [
            -0.3, -0.3, 0.0, 0.0, 1.0, // bottom left
            0.3, -0.3, 0.0, 1.0, 1.0, // bottom right
            -0.3, 0.3, 0.0, 0.0, 0.0, // top left
            0.3, 0.3, 0.0, 1.0, 0.0, // top right
        ];
        let indices: [u32; 6] = [
            0, 1, 2, // bottom left
            2, 1, 3, // top right
        ];
        let vao = gl.create_vertex_array().unwrap();
        let vbo = gl.create_buffer().unwrap();
        let ebo = gl.create_buffer().unwrap();

        gl.bind_vertex_array(Some(vao));

        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            bytemuck::cast_slice(&vertices[..]),
            glow::STATIC_DRAW,
        );

        // vertices
        gl.vertex_attrib_pointer_f32(
            0,
            3,
            glow::FLOAT,
            false,
            (std::mem::size_of::<f32>() * 5) as i32,
            0,
        );
        gl.enable_vertex_attrib_array(0);

        // uv
        gl.vertex_attrib_pointer_f32(
            1,
            2,
            glow::FLOAT,
            false,
            (std::mem::size_of::<f32>() * 5) as i32,
            (std::mem::size_of::<f32>() * 3) as i32,
        );
        gl.enable_vertex_attrib_array(1);

        gl.bind_buffer(glow::ARRAY_BUFFER, None);

        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
        gl.buffer_data_u8_slice(
            glow::ELEMENT_ARRAY_BUFFER,
            bytemuck::cast_slice(&indices[..]),
            glow::STATIC_DRAW,
        );

        gl.bind_vertex_array(None);
        (vbo, vao)
    };

    eloop.run(move |e, _target, cf| {
        *cf = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(16));
        match e {
            Event::LoopDestroyed => unsafe {
                gl.delete_buffer(vbo);
                gl.delete_vertex_array(vao);
                gl.delete_program(program);
            },
            Event::RedrawRequested(_) => {
                unsafe { draw_window(&gl, program, vao) };
                win.swap_buffers().unwrap();
            }
            Event::WindowEvent { event: ref e, .. } => {
                use glutin::event::WindowEvent;
                match e {
                    WindowEvent::Resized(sz) => {
                        win.resize(*sz);
                        unsafe { gl.viewport(0, 0, sz.width as i32, sz.height as i32) };
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

unsafe fn draw_window(
    gl: &glow::Context,
    program: glow::NativeProgram,
    vao: glow::NativeVertexArray,
) {
    gl.clear_color(0.46, 0.7, 0.76, 1.0);
    gl.clear(glow::COLOR_BUFFER_BIT);

    gl.use_program(Some(program));
    gl.bind_vertex_array(Some(vao));
    gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
    gl.bind_vertex_array(None);
}

unsafe fn compile_shader_from_source(
    gl: &glow::Context,
    shader_type: u32,
    src: &str,
) -> glow::NativeShader {
    let shader = gl.create_shader(shader_type).unwrap();
    gl.shader_source(shader, src);
    gl.compile_shader(shader);
    if !gl.get_shader_compile_status(shader) {
        panic!("{}", gl.get_shader_info_log(shader));
    }
    shader
}
