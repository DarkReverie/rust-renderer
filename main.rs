extern crate core_foundation;
extern crate core_graphics;
extern crate glium;
extern crate objc;
use glium::glutin::surface::WindowSurface;
use glium::implement_vertex;
use glium::winit;
use glium::{index::PrimitiveType, Display, IndexBuffer, Surface, VertexBuffer};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

enum ShapeIndices {
    NoIndices(glium::index::NoIndices),
    IndexBuffer(IndexBuffer<u16>),
}

implement_vertex!(Vertex, position);

fn draw_triangle(
    display: &Display<WindowSurface>,
) -> (Vec<Vertex>, glium::VertexBuffer<Vertex>, ShapeIndices) {
    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [-0.5, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.5],
    };
    let shape = vec![vertex1, vertex2, vertex3];
    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
    let indices = ShapeIndices::NoIndices(glium::index::NoIndices(PrimitiveType::TrianglesList));
    return (shape, vertex_buffer, indices);
}

fn draw_rectangle(
    display: &Display<WindowSurface>,
) -> (Vec<Vertex>, glium::VertexBuffer<Vertex>, ShapeIndices) {
    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [-0.5, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.5],
    };
    let vertex4 = Vertex {
        position: [0.5, 0.5],
    };

    let shape = vec![vertex1, vertex2, vertex3, vertex4];
    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
    let indices = ShapeIndices::IndexBuffer(
        IndexBuffer::new(
            display,
            PrimitiveType::TrianglesList,
            &[0u16, 1, 2, 1, 2, 3],
        )
        .unwrap(),
    );
    return (shape, vertex_buffer, indices);
}

fn draw_circle(
    display: &Display<WindowSurface>,
) -> (Vec<Vertex>, glium::VertexBuffer<Vertex>, ShapeIndices) {
    const POINTS_AMOUNT: usize = 100;
    const PI: f64 = 3.14159265359;
    const RADIUS: f64 = 0.5;

    let mut shape = Vec::with_capacity(POINTS_AMOUNT + 1);
    shape.push(Vertex {
        position: [0.0, 0.0],
    }); // Center of the circle

    for i in 0..POINTS_AMOUNT {
        let angle = (i as f64 / POINTS_AMOUNT as f64) * (2.0 * PI);
        let x_coord = RADIUS * angle.cos();
        let y_coord = RADIUS * angle.sin();
        shape.push(Vertex {
            position: [x_coord as f32, y_coord as f32],
        });
    }

    let mut indices = Vec::with_capacity(POINTS_AMOUNT * 3);
    for i in 0..POINTS_AMOUNT {
        indices.push(0);
        indices.push((i + 1) as u16);
        indices.push(((i + 1) % POINTS_AMOUNT + 1) as u16);
    }

    let vertex_buffer = VertexBuffer::new(display, &shape).unwrap();
    let index_buffer = IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices).unwrap();

    return (shape, vertex_buffer, ShapeIndices::IndexBuffer(index_buffer));
}
fn main() {
    println!("Hello, world!");
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);


    let vertex_shader_src = r#"
    #version 140

    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;

    let fragment_shader_src = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(0.5, 0.3, 0.8, 1.0);
    }
"#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut shape_index = 0;
    let shapes = vec![
        draw_triangle,
        draw_rectangle,
        draw_circle,
    ];

    let (shape, vertex_buffer, index_buffer) = shapes[shape_index](&display);

    let mut target = display.draw();
    target.clear_color(0.3, 0.7, 0.3, 0.0);
    match index_buffer {
        ShapeIndices::NoIndices(indices) => {
            target
                .draw(
                    &vertex_buffer,
                    &indices,
                    &program,
                    &glium::uniforms::EmptyUniforms,
                    &Default::default(),
                )
                .unwrap();
        }
        ShapeIndices::IndexBuffer(indices) => {
            target
                .draw(
                    &vertex_buffer,
                    &indices,
                    &program,
                    &glium::uniforms::EmptyUniforms,
                    &Default::default(),
                )
                .unwrap();
        }

    }
    target.finish().unwrap();
    let _ = event_loop.run(move |event, window_target|{
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                    println!("Resized to {:?}", window_size);
                }
                winit::event::WindowEvent::MouseInput { device_id, state, button } => {
                    if state == winit::event::ElementState::Pressed &&  button == winit::event::MouseButton::Left {
                        println!("Mouse clicked {}", shape_index);
                        shape_index = (shape_index + 1) % shapes.len();
                        println!("Shape index: {} ", shape_index);
                        let (shape, vertex_buffer, index_buffer) = shapes[shape_index](&display);
                        let mut target = display.draw();
                        target.clear_color(0.3, 0.7, 0.3, 0.0);
                        match index_buffer {
                            ShapeIndices::NoIndices(indices) => {
                                target
                                    .draw(
                                        &vertex_buffer,
                                        &indices,
                                        &program,
                                        &glium::uniforms::EmptyUniforms,
                                        &Default::default(),
                                    )
                                    .unwrap();
                            }
                            ShapeIndices::IndexBuffer(indices) => {
                                target
                                    .draw(
                                        &vertex_buffer,
                                        &indices,
                                        &program,
                                        &glium::uniforms::EmptyUniforms,
                                        &Default::default(),
                                    )
                                    .unwrap();
                            }
                        }
                        target.finish().unwrap();
                    }
                }
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                _ => (),
            },
            glium::winit::event::Event::AboutToWait => {
                window.request_redraw();
            }
            _ => (),
        };
    });
}
