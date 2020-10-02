#[allow(unused)]
mod gl;
mod input;
mod mixer;
mod platform;
mod texture_atlas;

use euclid::{
    default::{Rect, Transform2D, Vector2D},
    point2, size2, vec2,
};
use zerocopy::AsBytes;

use input::{InputEvent, Key};

fn main() {
    platform::run("My game", (800, 600), |gl_context: &mut gl::Context| {
        let vertex_shader = unsafe {
            gl_context
                .create_shader(gl::ShaderType::Vertex, include_str!("shaders/shader.vert"))
                .unwrap()
        };
        let fragment_shader = unsafe {
            gl_context
                .create_shader(
                    gl::ShaderType::Fragment,
                    include_str!("shaders/shader.frag"),
                )
                .unwrap()
        };

        let mut program = unsafe {
            gl_context
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
                .unwrap()
        };

        let logo_image = image::load_from_memory(include_bytes!("../assets/embla_logo.png"))
            .unwrap()
            .to_rgba();

        let mut texture = unsafe {
            gl_context
                .create_texture(gl::TextureFormat::RGBAFloat, 1024, 1024)
                .unwrap()
        };
        let mut atlas = texture_atlas::TextureAtlas::new((1024, 1024));

        let logo_width = logo_image.width();
        let logo_height = logo_image.height();
        let logo_uv = atlas
            .add_texture((logo_image.width(), logo_image.height()))
            .unwrap();
        unsafe {
            texture.write(
                logo_uv[0],
                logo_uv[1],
                logo_uv[2] - logo_uv[0],
                logo_uv[3] - logo_uv[1],
                &logo_image.into_raw(),
            )
        };

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

        let mut vertex_buffer = unsafe { gl_context.create_vertex_buffer().unwrap() };
        let mut position = point2(50., 100.);

        #[derive(Default)]
        struct Controls {
            up: bool,
            left: bool,
            down: bool,
            right: bool,
        }
        let mut controls = Controls::default();
        move |dt: f32, inputs: &[InputEvent], gl_context: &mut gl::Context| {
            for input in inputs {
                match input {
                    InputEvent::KeyDown(Key::W) => {
                        controls.up = true;
                    }
                    InputEvent::KeyUp(Key::W) => {
                        controls.up = false;
                    }
                    InputEvent::KeyDown(Key::A) => {
                        controls.left = true;
                    }
                    InputEvent::KeyUp(Key::A) => {
                        controls.left = false;
                    }
                    InputEvent::KeyDown(Key::S) => {
                        controls.down = true;
                    }
                    InputEvent::KeyUp(Key::S) => {
                        controls.down = false;
                    }
                    InputEvent::KeyDown(Key::D) => {
                        controls.right = true;
                    }
                    InputEvent::KeyUp(Key::D) => {
                        controls.right = false;
                    }
                    _ => {}
                }
            }

            let mut dir: Vector2D<f32> = vec2(0., 0.);
            if controls.up {
                dir.y += 1.;
            }
            if controls.down {
                dir.y -= 1.;
            }
            if controls.right {
                dir.x += 1.;
            }
            if controls.left {
                dir.x -= 1.;
            }
            if dir.length() > 0. {
                position += dir.normalize() * 50. * dt;
            }
            let rect = Rect::new(position, size2(logo_width as f32, logo_height as f32));
            let uv_rect = Rect::new(
                point2(logo_uv[0] as f32 / 1024., logo_uv[1] as f32 / 1024.),
                size2(
                    (logo_uv[2] - logo_uv[0]) as f32 / 1024.,
                    (logo_uv[3] - logo_uv[1]) as f32 / 1024.,
                ),
            );

            unsafe {
                gl_context.clear([0.0, 0.0, 0.0, 1.0]);

                vertex_buffer.write(&[
                    Vertex {
                        position: rect.min().to_array(),
                        uv: [uv_rect.min_x(), uv_rect.max_y()],
                    },
                    Vertex {
                        position: [rect.max_x(), rect.min_y()],
                        uv: uv_rect.max().to_array(),
                    },
                    Vertex {
                        position: [rect.min_x(), rect.max_y()],
                        uv: uv_rect.min().to_array(),
                    },
                    Vertex {
                        position: [rect.max_x(), rect.min_y()],
                        uv: uv_rect.max().to_array(),
                    },
                    Vertex {
                        position: rect.max().to_array(),
                        uv: [uv_rect.max_x(), uv_rect.min_y()],
                    },
                    Vertex {
                        position: [rect.min_x(), rect.max_y()],
                        uv: uv_rect.min().to_array(),
                    },
                ]);

                program.render_vertices(&vertex_buffer).unwrap();
            }
        }
    })
}

#[repr(C)]
#[derive(AsBytes)]
struct Vertex {
    position: [f32; 2],
    uv: [f32; 2],
}
