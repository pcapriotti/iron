use crate::game::Game;
use crate::glyphs::Glyphs;
use crate::graphics::{Instancing::*, VertexBuffer};
use crate::v2::V2;
use glow::HasContext;

pub struct Tile {
    vbo: VertexBuffer,
    cell_rects: VertexBuffer,
    glyph_indices: VertexBuffer,
    vao: glow::NativeVertexArray,
    program: glow::NativeProgram,
    texture: glow::NativeTexture,
    glyphs: Glyphs,
    num_instances: u32,
    width: u32,
    height: u32,
}

impl Tile {
    const GAP: f32 = 0.03;

    pub fn new(gl: &glow::Context) -> Self {
        let (vbo, cell_rects, glyph_indices, vao) = unsafe {
            let vertices: [f32; 8] = [
                0.0, 0.0, // bottom left
                1.0, 0.0, // bottom right
                0.0, 1.0, // top left
                1.0, 1.0, // top right
            ];
            let indices: [u32; 6] = [
                0, 1, 2, // bottom left
                2, 1, 3, // top right
            ];
            let vao = gl.create_vertex_array().unwrap();
            let ebo = gl.create_buffer().unwrap();

            gl.bind_vertex_array(Some(vao));

            // vertices
            let mut vbo = VertexBuffer::new(gl, 2, glow::FLOAT, ByVertex);
            vbo.enable(gl, 0);
            vbo.set_data(gl, &vertices[..], glow::STATIC_DRAW);

            // cell rects
            let cell_rects = VertexBuffer::new(gl, 4, glow::INT, ByInstance);
            cell_rects.enable(gl, 1);

            // glyph indices
            let glyph_indices = VertexBuffer::new(gl, 1, glow::INT, ByInstance);
            glyph_indices.enable(gl, 2);

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

        let atlas = glyphs.make_atlas(gl).unwrap();

        unsafe {
            gl.bind_buffer_base(
                glow::SHADER_STORAGE_BUFFER,
                0,
                Some(atlas.buffer()),
            );
        }

        Self {
            vbo,
            cell_rects,
            glyph_indices,
            vao,
            program,
            texture: atlas.texture(),
            glyphs,
            num_instances: 1,
            width: 0,
            height: 0,
        }
    }

    pub fn cleanup(&mut self, gl: &glow::Context) {
        unsafe {
            self.vbo.cleanup(gl);
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

    pub fn setup_grid(&mut self, gl: &glow::Context, game: &Game) {
        if self.width == 0 || self.height == 0 {
            return;
        }

        // let count = size.x * size.y;
        // self.num_instances = count as u32;
        self.num_instances = (game.width() * game.height()) as u32;

        let mut cell_rects: Vec<i32> = Vec::new();
        let mut glyph_indices: Vec<i32> = Vec::new();

        let (gap, cell_size) = if self.width < self.height {
            let gap = (self.width as f32 * Self::GAP) as i32;
            (
                gap,
                ((self.width as f32 - gap as f32) / game.width() as f32) as i32
                    - gap,
            )
        } else {
            let gap = (self.height as f32 * Self::GAP) as i32;
            (
                gap,
                ((self.height as f32 - gap as f32) / game.height() as f32)
                    as i32
                    - gap,
            )
        };

        for y in 0..game.height() as i32 {
            for x in 0..game.width() as i32 {
                let r = [
                    gap + x * (gap + cell_size),
                    gap + y * (gap + cell_size),
                    cell_size,
                    cell_size,
                ];
                cell_rects.extend_from_slice(&r);
                glyph_indices.push(65 + x + 4 * y);
            }
        }

        unsafe {
            self.cell_rects
                .set_data(gl, &cell_rects[..], glow::DYNAMIC_DRAW);
            self.glyph_indices.set_data(
                gl,
                &glyph_indices[..],
                glow::DYNAMIC_DRAW,
            );

            gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
            self.glyphs.upload_atlas(&gl).unwrap();
            gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }

    pub fn resize(&mut self, gl: &glow::Context, width: u32, height: u32) {
        self.width = width;
        self.height = height;
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
