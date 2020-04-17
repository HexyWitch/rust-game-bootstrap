#[allow(unused)]
mod gl;
mod runtime;

use euclid::{
    default::{Rect, Transform2D},
    point2, size2, vec2,
};
use zerocopy::AsBytes;

fn main() {
    runtime::run("My game", (800, 600), |gl_context: &mut gl::Context| {
        let vertex_shader = gl_context
            .create_shader(gl::ShaderType::Vertex, include_str!("shaders/shader.vert"))
            .unwrap();
        let fragment_shader = gl_context
            .create_shader(
                gl::ShaderType::Fragment,
                include_str!("shaders/shader.frag"),
            )
            .unwrap();

        let mut program = gl_context
            .create_program(&gl::ProgramDescriptor {
                vertex_shader: &vertex_shader,
                fragment_shader: &fragment_shader,
                uniforms: &[
                    gl::UniformEntry {
                        name: "u_transform",
                        ty: gl::UniformType::Mat3,
                    },
                    gl::UniformEntry {
                        name: "u_texture",
                        ty: gl::UniformType::Texture,
                    },
                ],
                vertex_format: gl::VertexFormat {
                    stride: std::mem::size_of::<Vertex>(),
                    attributes: &[
                        gl::VertexAttribute {
                            name: "a_pos",
                            ty: gl::VertexAttributeType::Float,
                            size: 2,
                            offset: 0,
                        },
                        gl::VertexAttribute {
                            name: "a_uv",
                            ty: gl::VertexAttributeType::Float,
                            size: 2,
                            offset: 2 * 4,
                        },
                    ],
                },
            })
            .unwrap();

        let logo_image = image::load_from_memory(include_bytes!("../assets/embla_logo.png"))
            .unwrap()
            .to_rgba();

        let mut texture = gl_context
            .create_texture(
                gl::TextureFormat::RGBAFloat,
                logo_image.width(),
                logo_image.height(),
            )
            .unwrap();

        let logo_width = logo_image.width();
        let logo_height = logo_image.height();
        texture.write(
            0,
            0,
            logo_image.width(),
            logo_image.height(),
            &logo_image.into_raw(),
        );

        let transform = Transform2D::create_scale(1.0 / 800.0, 1.0 / 600.0)
            .post_scale(2., 2.)
            .post_translate(vec2(-1.0, -1.0));
        program
            .set_uniform(
                0,
                gl::Uniform::Mat3([
                    [transform.m11, transform.m12, 0.0],
                    [transform.m21, transform.m22, 0.0],
                    [transform.m31, transform.m32, 1.0],
                ]),
            )
            .unwrap();
        program
            .set_uniform(1, gl::Uniform::Texture(&texture))
            .unwrap();

        let mut vertex_buffer = gl_context.create_vertex_buffer().unwrap();

        move |gl_context: &mut gl::Context| {
            gl_context.clear([0.0, 0.0, 0.0, 1.0]);

            let rect = Rect::new(
                point2(50., 100.),
                size2(logo_width as f32, logo_height as f32),
            );

            vertex_buffer.write(&[
                Vertex {
                    position: rect.min().to_array(),
                    uv: [0.0, 1.0],
                },
                Vertex {
                    position: [rect.max_x(), rect.min_y()],
                    uv: [1.0, 1.0],
                },
                Vertex {
                    position: [rect.min_x(), rect.max_y()],
                    uv: [0.0, 0.0],
                },
                Vertex {
                    position: [rect.max_x(), rect.min_y()],
                    uv: [1.0, 1.0],
                },
                Vertex {
                    position: rect.max().to_array(),
                    uv: [1.0, 0.0],
                },
                Vertex {
                    position: [rect.min_x(), rect.max_y()],
                    uv: [0.0, 0.0],
                },
            ]);

            program.render_vertices(&vertex_buffer).unwrap();
        }
    })
}

#[repr(C)]
#[derive(AsBytes)]
struct Vertex {
    position: [f32; 2],
    uv: [f32; 2],
}
