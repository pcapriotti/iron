use crate::glyphs::Glyphs;
use crate::v2::V2;
use glow::HasContext;

const GAP: f32 = 0.1;
const MARGIN: f32 = 0.08;

pub struct Tile {
    vbo: glow::NativeBuffer,
    offsets: glow::NativeBuffer,
    uv_rects: glow::NativeBuffer,
    rects: glow::NativeBuffer,
    vao: glow::NativeVertexArray,
    program: glow::NativeProgram,
    texture: glow::NativeTexture,
    glyphs: Glyphs,
    num_instances: u32,
}

impl Tile {
    pub fn new(gl: &glow::Context) -> Self {
        let (vbo, offsets, uv_rects, rects, vao) = unsafe {
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

            // offsets
            let offsets_array: [f32; 2] = [0.0, 0.0];
            let offsets = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(offsets));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&offsets_array[..]),
                glow::STATIC_DRAW,
            );
            gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, 0, 0);
            gl.vertex_attrib_divisor(2, 1);
            gl.enable_vertex_attrib_array(2);

            // uv rects
            let uv_rects_array: [f32; 4] = [0.0, 0.0, 0.5, 0.5];
            let uv_rects = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(uv_rects));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&uv_rects_array[..]),
                glow::DYNAMIC_DRAW,
            );
            gl.vertex_attrib_pointer_f32(3, 4, glow::FLOAT, false, 0, 0);
            gl.vertex_attrib_divisor(3, 1);
            gl.enable_vertex_attrib_array(3);

            // rects
            let rects_array: [i32; 4] = [0, 0, 10, 10];
            let rects = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(rects));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&rects_array[..]),
                glow::DYNAMIC_DRAW,
            );
            gl.vertex_attrib_pointer_f32(4, 4, glow::INT, false, 0, 0);
            gl.vertex_attrib_divisor(4, 1);
            gl.enable_vertex_attrib_array(4);

            gl.bind_buffer(glow::ARRAY_BUFFER, None);

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&indices[..]),
                glow::STATIC_DRAW,
            );

            gl.bind_vertex_array(None);
            (vbo, offsets, uv_rects, rects, vao)
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

        let texture = unsafe {
            let texture = gl.create_texture().unwrap();

            {
                gl.use_program(Some(program));
                let loc = gl.get_uniform_location(program, "t");
                gl.uniform_1_i32(loc.as_ref(), 0);
                gl.use_program(None);
            }

            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );

            let width = 1024;
            let height = 1024;

            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                width,
                height,
                0,
                glow::RED,
                glow::UNSIGNED_BYTE,
                Some(&vec![0xff; (width * height) as usize]),
            );
            gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);
            glyphs.upload_atlas(gl).unwrap();
            gl.bind_texture(glow::TEXTURE_2D, None);

            texture
        };

        Self {
            vbo,
            offsets,
            uv_rects,
            rects,
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

    pub fn setup_grid(&mut self, gl: &glow::Context, size: &V2<u32>) {
        let count = size.x * size.y;
        self.num_instances = count as u32;

        let mut offsets: Vec<f32> = Vec::new();
        let mut uv_rects: Vec<f32> = Vec::new();
        let mut rects: Vec<i32> = Vec::new();
        let mut c = 'a';

        for x in 0..size.x {
            for y in 0..size.y {
                offsets.push(0.2 * x as f32);
                offsets.push(0.2 * y as f32);

                if let Some((uv_rect, rect)) = self.glyphs.rect_for(c).unwrap() {
                    uv_rects.push(uv_rect.min.x);
                    uv_rects.push(uv_rect.min.y);
                    uv_rects.push(uv_rect.width());
                    uv_rects.push(uv_rect.height());

                    println!(
                        "{}: ({}, {}) {} × {}",
                        c,
                        rect.min.x,
                        rect.min.y,
                        rect.width(),
                        rect.height()
                    );
                    rects.push(rect.min.x);
                    rects.push(rect.min.y);
                    rects.push(rect.width());
                    rects.push(rect.height());
                } else {
                    uv_rects.push(0.0);
                    uv_rects.push(0.0);
                    uv_rects.push(0.0);
                    uv_rects.push(0.0);
                }

                c = (c as u8 + 1) as char;
            }
        }

        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.offsets));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&offsets[..]),
                glow::STATIC_DRAW,
            );

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.uv_rects));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&uv_rects[..]),
                glow::DYNAMIC_DRAW,
            );

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.rects));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&rects[..]),
                glow::DYNAMIC_DRAW,
            );
        }
    }

    pub fn set_scale(&mut self, gl: &glow::Context, width: u32, height: u32, size: &V2<u32>) {
        let ratio = width as f32 / height as f32;
        let scale = if ratio > 1.0 {
            V2::new(1.0 / ratio, 1.0)
        } else {
            V2::new(1.0, ratio)
        };

        unsafe { gl.use_program(Some(self.program)) };
        unsafe {
            let loc = gl.get_uniform_location(self.program, "scale");
            gl.uniform_2_f32(
                loc.as_ref(),
                scale.x * 2.0 * (1.0 - MARGIN) / (size.x as f32 * (1.0 + GAP) - GAP),
                scale.y * 2.0 * (1.0 - MARGIN) / (size.y as f32 * (1.0 + GAP) - GAP),
            );
        }
        unsafe {
            let loc = gl.get_uniform_location(self.program, "resolution");
            gl.uniform_2_u32(loc.as_ref(), width, height);
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
