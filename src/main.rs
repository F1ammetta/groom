#[macro_use]
extern crate glium;
use std::fs;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use cgmath::Rotation3;
use cgmath::{Deg, Quaternion, Vector3};
use crossbeam::channel::unbounded;
use glium::IndexBuffer;
use glium::Surface;
use glium::VertexBuffer;
use glium::winit::event_loop::ControlFlow;
use glium::winit::event_loop::EventLoop;

mod camera;
mod drawing;
mod events;
mod geo;
mod input;
mod mat;
mod mesh;
mod phys;
mod threading;
mod vx;

use camera::CamParams;
use phys::PhysicsWorld;
use threading::PhysicsMessage;

#[allow(deprecated)]
fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let run_setter = running.clone();

    ctrlc::set_handler(move || {
        run_setter.store(false, Ordering::SeqCst);
    })
    .expect("Error Setting Exit handler");

    let event_loop = EventLoop::builder().build().unwrap();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_vsync(false)
        .build(&event_loop);

    let (tx, rx) = unbounded::<PhysicsMessage>();

    let vertex_shader = glsl!("vertex");
    let fragment_shader = glsl!("fragment");

    let world = PhysicsWorld::new();

    let (base_vertices, base_indices) = world.get_base_mesh();

    // Create base mesh buffers (do this once)
    let v_buf = VertexBuffer::new(&display, base_vertices).unwrap();
    let i_buf = IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        base_indices,
    )
    .unwrap();

    let mut target = display.draw();
    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
    target.finish().unwrap();

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

    let program =
        glium::Program::from_source(&display, &vertex_shader, &fragment_shader, None).unwrap();

    let position = Vector3::new(0.0, 0.0, 2.0);
    let orientation = Quaternion::from_angle_y(Deg(180.0)); // Looking backward

    let fov = 60.0;

    let size = window.inner_size();

    let ar = size.width as f32 / size.height as f32;

    let mut cam = CamParams {
        pos: position,
        ori: orientation,
        fov: fov,
        ar: ar,
    };

    let mut matrix = camera::camera_matrix(cam.pos, cam.ori, cam.fov, cam.ar, 0.1, 100.0);

    let mut l_t = Instant::now();

    let physics_run = running.clone();

    let physics_thread = std::thread::spawn(move || {
        threading::phys_start(physics_run, tx);
    });

    let _ = event_loop.run(move |event, window_target| {
        if !running.load(Ordering::SeqCst) {
            window_target.exit();
        }

        window_target.set_control_flow(ControlFlow::Poll);

        events::handle(
            &mut l_t,
            event,
            window_target,
            &window,
            &display,
            &mut cam,
            &rx,
            &running,
            |ins_buffer| {
                drawing::draw_shape(
                    &display,
                    &i_buf,
                    &program,
                    (&v_buf, ins_buffer),
                    &matrix,
                    &draw_params,
                );
            },
        );

        matrix = camera::camera_matrix(cam.pos, cam.ori, cam.fov, cam.ar, 0.1, 100.0);
    });
    physics_thread.join().unwrap();
}
