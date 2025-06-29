use core::panic;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use crate::input;
use crate::phys::InstanceData;
use crate::threading::PhysicsMessage;
use cgmath::Rotation;
use crossbeam::channel::Receiver;
use glium::VertexBuffer;
use glium::glutin::surface::WindowSurface;
use glium::vertex::PerInstance;
use glium::winit::dpi::LogicalPosition;
use glium::winit::event::{Event, WindowEvent};
use glium::winit::event_loop::ActiveEventLoop;
use glium::winit::keyboard::KeyCode;
use glium::winit::window::Window;

use crate::CamParams;

pub fn handle<F: FnOnce(PerInstance)>(
    l_t: &mut Instant,
    event: Event<()>,
    window_target: &ActiveEventLoop,
    window: &Window,
    display: &glium::Display<WindowSurface>,
    cam: &mut CamParams,
    physics_rx: &Receiver<PhysicsMessage>,
    running: &Arc<AtomicBool>,
    draw_cb: F,
) {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(window_size) => {
                display.resize(window_size.into());

                cam.ar = window_size.width as f32 / window_size.height as f32;
            }
            WindowEvent::CloseRequested => {
                running.store(false, Ordering::SeqCst);
                window_target.exit();
            }
            WindowEvent::CursorMoved { position, .. } => {
                let size_x = window.inner_size().width;
                let size_y = window.inner_size().height;
                let pos = (
                    ((position.x / size_x as f64) * 2.0) - 1.0,
                    (2.0 - (position.y / size_y as f64) * 2.0) - 1.0,
                );

                let mut d_pos = (pos.0 - cam.last_c_pos.0, pos.1 - cam.last_c_pos.1);

                if d_pos.0.abs() < f64::EPSILON && d_pos.1.abs() < f64::EPSILON {
                    let edge_movement_amount = 0.01;

                    if pos.0 <= -0.99 {
                        d_pos.0 = -edge_movement_amount;
                    } else if pos.0 >= 0.99 {
                        d_pos.0 = edge_movement_amount;
                    }

                    if pos.1 <= -0.99 {
                        d_pos.1 = -edge_movement_amount;
                    } else if pos.1 >= 0.99 {
                        d_pos.1 = edge_movement_amount;
                    }
                }

                cam.last_c_pos = pos;

                input::rotate_cam(d_pos, cam);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                input::key_handle(event.physical_key, |vec| {
                    let vec = cam.ori.rotate_vector(vec);
                    cam.pos -= vec;
                });
                if event.physical_key == KeyCode::Escape {
                    running.store(false, Ordering::SeqCst);
                }
            }
            WindowEvent::RedrawRequested => {
                // t = start.elapsed().as_secs_f32();
                let dt = l_t.elapsed().as_secs_f32();
                *l_t = Instant::now();

                println!("{:.2} Fps", 1.0 / dt);

                // (position, orientation) = rotate_around_origin_xz_dt(dt, position, orientation);

                // TODO: Don't block
                let Ok(message) = physics_rx.recv() else {
                    panic!("Failed to communicate with physics thread");
                };

                let instance_data = match message {
                    PhysicsMessage::InstanceData(data) => data,
                };

                // let instance_data: Vec<InstanceData> = Vec::new();

                // let instance_data = world.get_instance_data();

                // Create instance buffer (updated each frame)
                let Ok(instance_buffer) = VertexBuffer::new(display, &instance_data) else {
                    panic!("Error creating instance vertex buffer");
                };

                let Ok(instance_buffer) = instance_buffer.per_instance() else {
                    panic!("Error creating instance buffer");
                };

                draw_cb(instance_buffer);
            }

            _ => (),
        },

        Event::AboutToWait => {
            window.request_redraw();
        }
        _ => (),
    }
}
