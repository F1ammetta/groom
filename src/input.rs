use cgmath::Vector3;
use glium::winit::keyboard::PhysicalKey::{Code, Unidentified};
use glium::winit::keyboard::{KeyCode, PhysicalKey};

pub fn key_handle<F: FnOnce(Vector3<f32>)>(key: PhysicalKey, cb: F) {
    match key {
        Code(code) => match code {
            KeyCode::KeyS => {
                cb(Vector3::new(0.0, 0.0, 0.5));
            }
            KeyCode::KeyW => {
                cb(Vector3::new(0.0, 0.0, -0.5));
            }
            KeyCode::KeyD => {
                cb(Vector3::new(0.5, 0.0, 0.0));
            }
            KeyCode::KeyA => {
                cb(Vector3::new(-0.5, 0.0, 0.0));
            }
            _ => {}
        },
        Unidentified(_) => {}
    }
}
