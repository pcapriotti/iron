use crate::glyphs::Glyphs;
use crate::v2::V2;
use glow::HasContext;

const GAP: f32 = 0.1;
const MARGIN: f32 = 0.08;

pub struct Tile {
    vbo: glow::NativeBuffer,
    cell_rects: glow::NativeBuffer,
    glyph_indices: glow::NativeBuffer,
    vao: glow::NativeVertexArray,
    program: glow::NativeProgram,
    texture: glow::NativeTexture,
    glyphs: Glyphs,
    num_instances: u32,
}

impl Tile {
    pub fn new(gl: &glow::Context) -> Self {
        let (vbo, cell_rects, glyph_indices, vao) = unsafe {
            let vertices: [f32; 16] = [
                0.0, 0.0, 0.0, 1.0, // bottom left
                1.0, 0.0, 1.0, 1.0, // bottom right
                0.0, 1.0, 0.0, 0.0, // top left
                1.0, 1.0, 1.0, 0.0, // top right
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
                (std::mem::size_of::<f32>() * 4) as i32,
                0,
            );
            gl.enable_vertex_attrib_array(0);

            // uv
            gl.vertex_attrib_pointer_f32(
                1,
                2,
                glow::FLOAT,
                false,
                (std::mem::size_of::<f32>() * 4) as i32,
                (std::mem::size_of::<f32>() * 2) as i32,
            );
            gl.enable_vertex_attrib_array(1);

            // cell rects
            let cell_rect_array: [i32; 4] = [0, 0, 0, 0];
            let cell_rects = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(cell_rects));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&cell_rect_array[..]),
                glow::STATIC_DRAW,
            );
            gl.vertex_attrib_pointer_i32(2, 4, glow::INT, 0, 0);
            gl.vertex_attrib_divisor(2, 1);
            gl.enable_vertex_attrib_array(2);

            // glyph indices
            let glyph_index_array: [i32; 1] = [0];
            let glyph_indices = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(glyph_indices));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&glyph_index_array[..]),
                glow::STATIC_DRAW,
            );
            gl.vertex_attrib_pointer_i32(3, 1, glow::INT, 0, 0);
            gl.vertex_attrib_divisor(3, 1);
            gl.enable_vertex_attrib_array(3);

            gl.bind_buffer(glow::ARRAY_BUFFER, None);

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&indices[..]),
                glow::STATIC_DRAW,
            );

            gl.bind_vertex_array(None);
            (vbo, cell_rects, glyph_indices, vao)
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

        let mut glyphs = Glyphs::new().unwrap();

        let (texture, buffer) = glyphs.make_atlas(gl).unwrap();

        unsafe {
            gl.bind_buffer_base(glow::SHADER_STORAGE_BUFFER, 0, Some(buffer));
        }

        Self {
            vbo,
            cell_rects,
            glyph_indices,
            vao,
            program,
            texture,
            glyphs,
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
            gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
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

    pub fn setup_grid(&mut self, gl: &glow::Context, _size: &V2<u32>) {
        // let count = size.x * size.y;
        // self.num_instances = count as u32;
        self.num_instances = 15;

        let mut cell_rects: Vec<i32> = Vec::new();
        let mut glyph_indices: Vec<i32> = Vec::new();

        for i in 0..self.num_instances as i32 {
            cell_rects.extend_from_slice(&[10 + i * 90, 400, 90, 250]);
            glyph_indices.push(i + 60);
        }

        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.cell_rects));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&cell_rects[..]),
                glow::DYNAMIC_DRAW,
            );
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.glyph_indices));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&glyph_indices[..]),
                glow::DYNAMIC_DRAW,
            );

            gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
            self.glyphs.upload_atlas(&gl);
            gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }

    pub fn resize(&mut self, gl: &glow::Context, width: u32, height: u32) {
        unsafe {
            gl.use_program(Some(self.program));
            let loc = gl.get_uniform_location(self.program, "viewport");
            gl.uniform_4_i32(loc.as_ref(), 0, 0, width as i32, height as i32);
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
