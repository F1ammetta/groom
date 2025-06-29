use cgmath::{Deg, InnerSpace, Quaternion, Rad, Rotation, Rotation3, Vector3};
use glium::winit::keyboard::PhysicalKey::{Code, Unidentified};
use glium::winit::keyboard::{KeyCode, PhysicalKey};

use crate::camera::CamParams;

pub fn rotate_cam(d_pos: (f64, f64), cam: &mut CamParams) {
    let d_pos = (d_pos.0 as f32, d_pos.1 as f32);
    let sens = 0.8;

    let yaw_rotation = Quaternion::from_angle_y(Rad(-d_pos.0 * sens));
    let pitch_rotation = Quaternion::from_angle_x(Rad(-d_pos.1 * sens));

    cam.ori = yaw_rotation * cam.ori * pitch_rotation;

    cam.ori = cam.ori.normalize();
}

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
