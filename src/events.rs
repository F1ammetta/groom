use crate::input;
use crate::threading::PhysicsMessage;
use crossbeam::channel::Receiver;
use glium::VertexBuffer;
use glium::glutin::surface::WindowSurface;
use glium::vertex::PerInstance;
use glium::winit::event::{Event, WindowEvent};
use glium::winit::event_loop::ActiveEventLoop;
use glium::winit::window::Window;

use crate::CamParams;

pub fn handle<F: FnOnce(PerInstance)>(
    event: Event<()>,
    window_target: &ActiveEventLoop,
    window: &Window,
    display: &glium::Display<WindowSurface>,
    cam: &mut CamParams,
    physics_rx: &Receiver<PhysicsMessage>,
    draw_cb: F,
) {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(window_size) => {
                display.resize(window_size.into());

                cam.ar = window_size.width as f32 / window_size.height as f32;
            }
            WindowEvent::CloseRequested => window_target.exit(),
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                input::key_handle(event.physical_key, |vec| {
                    cam.pos += vec;
                });
            }
            WindowEvent::RedrawRequested => {
                // t = start.elapsed().as_secs_f32();
                // dt = l_t.elapsed().as_secs_f32();
                // l_t = Instant::now();

                // (position, orientation) = rotate_around_origin_xz_dt(dt, position, orientation);

                let message = physics_rx.recv().unwrap();

                let instance_data = message.instance_data;

                // let instance_data = world.get_instance_data();

                // Create instance buffer (updated each frame)
                let instance_buffer = VertexBuffer::new(display, &instance_data).unwrap();

                draw_cb(instance_buffer.per_instance().unwrap());
            }

            _ => (),
        },
        Event::AboutToWait => {
            window.request_redraw();
        }
        _ => (),
    }
}
