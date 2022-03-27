use glow::HasContext;

pub struct Tile {
    vbo: glow::NativeBuffer,
    offsets: glow::NativeBuffer,
    vao: glow::NativeVertexArray,
    program: glow::NativeProgram,
    num_instances: u32,
}

impl Tile {
    pub fn new(gl: &glow::Context) -> Self {
        let (vbo, offsets, vao) = unsafe {
            let vertices: [f32; 20] = [
                0.0, 0.0, 0.0, 0.0, 1.0, // bottom left
                1.0, 0.0, 0.0, 1.0, 1.0, // bottom right
                0.0, 1.0, 0.0, 0.0, 0.0, // top left
                1.0, 1.0, 0.0, 1.0, 0.0, // top right
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

            // offsets
            let offsets_array: [f32; 2] = [0.0, 0.0];
            let offsets = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(offsets));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&offsets_array[..]),
                glow::DYNAMIC_DRAW,
            );
            gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, 0, 0);
            gl.vertex_attrib_divisor(2, 1);
            gl.enable_vertex_attrib_array(2);

            gl.bind_buffer(glow::ARRAY_BUFFER, None);

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&indices[..]),
                glow::STATIC_DRAW,
            );

            gl.bind_vertex_array(None);
            (vbo, offsets, vao)
        };

        let program = unsafe {
            let vert = compile_shader_from_source(
                &gl,
                glow::VERTEX_SHADER,
                include_bytes!("../shaders/tile.v.glsl"),
            );

            let frag = compile_shader_from_source(
                &gl,
                glow::FRAGMENT_SHADER,
                include_bytes!("../shaders/tile.f.glsl"),
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
        Self {
            vbo,
            offsets,
            vao,
            program,
            num_instances: 1,
        }
    }

    pub fn cleanup(self, gl: &glow::Context) {
        unsafe {
            gl.delete_buffer(self.vbo);
            gl.delete_vertex_array(self.vao);
            gl.delete_program(self.program);
        }
    }

    pub fn render(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(Some(self.program));
            gl.bind_vertex_array(Some(self.vao));
            gl.draw_elements_instanced(
                glow::TRIANGLES,
                6,
                glow::UNSIGNED_INT,
                0,
                self.num_instances as i32,
            );
            gl.bind_vertex_array(None);
        }
    }
}

unsafe fn compile_shader_from_source(
    gl: &glow::Context,
    shader_type: u32,
    src: &[u8],
) -> glow::NativeShader {
    let shader = gl.create_shader(shader_type).unwrap();
    gl.shader_source(shader, std::str::from_utf8(src).unwrap());
    gl.compile_shader(shader);
    if !gl.get_shader_compile_status(shader) {
        panic!("{}", gl.get_shader_info_log(shader));
    }
    shader
}
