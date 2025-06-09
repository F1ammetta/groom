#[macro_use]
extern crate glium;
use std::fs;

use cgmath::InnerSpace;
use cgmath::Rotation;
use cgmath::Rotation3;
use glium::DrawParameters;
use glium::IndexBuffer;
use glium::Program;
use glium::Surface;
use glium::VertexBuffer;
use glium::glutin::surface::WindowSurface;
use glium::winit::event::KeyEvent;
use glium::winit::event::{Event, WindowEvent};
use glium::winit::event_loop::EventLoop;
use glium::winit::keyboard::KeyCode;
use glium::winit::keyboard::PhysicalKey::{Code, Unidentified};
use phys::InstanceData;
use phys::PhysicsWorld;
use std::time::Instant;

use cgmath::{Deg, Matrix3, Quaternion, Rad, Vector3};

fn rotate_around_origin_xz_dt(
    dt: f32,
    last_pos: Vector3<f32>,
    _last_ori: Quaternion<f32>,
) -> (Vector3<f32>, Quaternion<f32>) {
    // Rotate the position around Y-axis (XZ-plane rotation)
    let rot = Matrix3::from_angle_y(Rad(dt));
    let new_pos = rot * last_pos;

    // Direction to look at: from new position toward origin
    let forward = (-new_pos).normalize();
    let up = Vector3::unit_y();

    // Compute a right-handed coordinate frame (right, up, forward)
    let right = up.cross(forward).normalize();
    let corrected_up = forward.cross(right).normalize();

    // Create rotation matrix from the basis vectors
    let rotation_matrix = Matrix3::from_cols(right, corrected_up, forward);
    let orientation = Quaternion::from(rotation_matrix);

    (new_pos, orientation)
}

mod camera;
mod geo;
mod mat;
mod phys;
mod vx;
use mat::Mat4;
use phys::Particle;
use vx::Vx;

fn draw_shape(
    display: &glium::Display<WindowSurface>,
    indices: &IndexBuffer<u16>,
    program: &Program,
    v_buf: &VertexBuffer<Vx>,
    instance_buffer: &VertexBuffer<InstanceData>,
    matrix: &Mat4,
    params: &DrawParameters,
) {
    // *t += 0.02;

    // let mat = mat![
    //     t.cos(), -t.sin(), 0.0, 0.0;//t.cos() * 0.3;
    //     t.sin(), t.cos(),  0.0, 0.0;//t.sin() * 0.3;
    //     0.0,     0.0,      1.0, 0.0;
    //     0.0,     0.0,      0.0, 1.0
    // ];

    let mut target = display.draw();
    target.clear_color_and_depth((0.0 / 255.0, 120.0 / 255.0, 140.0 / 255.0, 1.0), 1.0);
    target
        .draw(
            (v_buf, instance_buffer.per_instance().unwrap()),
            indices,
            program,
            &uniform! {
                matrix: *matrix
            },
            params,
        )
        .unwrap();
    target.finish().unwrap();
}

fn main() {
    // 1. The **winit::EventLoop** for handling events.
    let event_loop = EventLoop::builder().build().unwrap();
    // 2. Create a glutin context and glium Display
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    // window.set_resizable(true);

    let vertex_shader_src = fs::read_to_string("shaders/vertex.glsl").unwrap();

    let fragment_shader_src = fs::read_to_string("shaders/fragment.glsl").unwrap();

    let mut world = PhysicsWorld::new();

    world.add_particle(part!(0.0,0.0, 0.0 ; 1 ; 0.1));
    world.add_particle(part!(0.5,0.0,0.0 ; 1 ; 0.1));
    world.add_particle(part!(-0.5,0.0,0.0 ; 1 ; 0.1));

    let (base_vertices, base_indices) = world.get_base_mesh();

    // Create base mesh buffers (do this once)
    let v_buf = VertexBuffer::new(&display, base_vertices).unwrap();
    let i_buf = IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        base_indices,
    )
    .unwrap();

    // In your render loop:
    let mut instance_data = world.get_instance_data();

    // Create instance buffer (updated each frame)
    let mut instance_buffer = VertexBuffer::new(&display, &instance_data).unwrap();

    let mut target = display.draw();
    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

    let program =
        glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None)
            .unwrap();

    target.finish().unwrap();

    let mut target = display.draw();

    target.clear_color_and_depth((0.0 / 255.0, 120.0 / 255.0, 140.0 / 255.0, 1.0), 1.0);

    let draw_params = glium::DrawParameters {
        polygon_mode: glium::PolygonMode::Fill,
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        backface_culling: glium::BackfaceCullingMode::CullingDisabled,
        ..Default::default()
    };

    let mut position = Vector3::new(0.0, 0.0, 2.0);
    // let orientation = Quaternion::from_angle_x(Deg(90.0)); // Looking backward
    let mut orientation = Quaternion::from_angle_y(Deg(180.0)); // Looking backward

    let mut matrix = camera::camera_matrix(
        position,
        orientation,
        60.0,       // FOV in degrees
        16.0 / 9.0, // aspect ratio
        0.1,        // near plane
        100.0,      // far plane
    );

    // Draw with instancing
    target
        .draw(
            (&v_buf, instance_buffer.per_instance().unwrap()),
            &i_buf,
            &program,
            &uniform! { matrix: matrix },
            &DrawParameters::default(),
        )
        .unwrap();

    // target
    //     .draw(
    //         &v_buf,
    //         &i_buf,
    //         &program,
    //         &uniform! {matrix: matrix, },
    //         &draw_params,
    //     )
    //     .unwrap();

    target.finish().unwrap();

    let start = Instant::now();
    let mut t: f32 = 0.0;
    let mut dt: f32 = 0.0;
    let fov = 60.0;

    let mut ar = 0.0;
    let mut l_t = Instant::now();

    let _ = event_loop.run(move |event, window_target| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(window_size) => {
                display.resize(window_size.into());

                ar = window_size.width as f32 / window_size.height as f32;

                matrix = camera::camera_matrix(
                    position,
                    orientation,
                    fov,   // FOV in degrees
                    ar,    // aspect ratio
                    0.1,   // near plane
                    100.0, // far plane
                );
            }
            WindowEvent::CloseRequested => window_target.exit(),
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                match event.physical_key {
                    Code(code) => match code {
                        KeyCode::KeyS => {
                            position += Vector3::new(0.0, 0.0, -0.5);
                        }
                        KeyCode::KeyW => {
                            position += Vector3::new(0.0, 0.0, 0.5);
                        }
                        KeyCode::KeyD => {
                            position += Vector3::new(-0.5, 0.0, 0.0);
                        }
                        KeyCode::KeyA => {
                            position += Vector3::new(0.5, 0.0, 0.0);
                        }
                        _ => {}
                    },
                    Unidentified(_) => {}
                };
                matrix = camera::camera_matrix(
                    position,
                    orientation,
                    fov, // FOV in degrees
                    ar,
                    0.1,   // near plane
                    100.0, // far plane
                );
            }
            WindowEvent::MouseWheel {
                device_id: _,
                delta,
                phase: _,
            } => {
                let dz = match delta {
                    glium::winit::event::MouseScrollDelta::LineDelta(_, b) => b,
                    _ => 0.0,
                } * 0.5;

                position += Vector3::new(0.0, 0.0, -dz);

                matrix = camera::camera_matrix(
                    position,
                    orientation,
                    fov, // FOV in degrees
                    ar,
                    0.1,   // near plane
                    100.0, // far plane
                );
            }
            WindowEvent::RedrawRequested => {
                t = start.elapsed().as_secs_f32();
                dt = l_t.elapsed().as_secs_f32();
                println!("{:.2} fps", 1.0 / dt);
                l_t = Instant::now();

                (position, orientation) = rotate_around_origin_xz_dt(dt, position, orientation);

                matrix = camera::camera_matrix(
                    position,
                    orientation,
                    fov, // FOV in degrees
                    ar,
                    0.1,   // near plane
                    100.0, // far plane
                );

                instance_data = world.get_instance_data();

                // Create instance buffer (updated each frame)
                instance_buffer = VertexBuffer::new(&display, &instance_data).unwrap();

                draw_shape(
                    &display,
                    &i_buf,
                    &program,
                    &v_buf,
                    &instance_buffer,
                    &matrix,
                    &draw_params,
                );
            }

            _ => (),
        },
        Event::AboutToWait => {
            window.request_redraw();
        }
        _ => (),
    });
}
