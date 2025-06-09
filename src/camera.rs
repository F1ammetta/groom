use cgmath::{
    EuclideanSpace, Matrix, Matrix4, Point3, Quaternion, Rad, Rotation, Vector3, perspective,
};

use crate::mat::Mat4;

/// Returns the view-projection matrix from the camera's position and orientation.
pub fn camera_matrix(
    position: Vector3<f32>,
    orientation: Quaternion<f32>,
    fov_degrees: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
) -> Mat4 {
    // Create the perspective (projection) matrix
    let projection = perspective(Rad(fov_degrees.to_radians()), aspect_ratio, near, far);

    // Compute the view matrix by applying the inverse of the camera transform
    let forward = orientation.rotate_vector(Vector3::unit_z());
    let up = orientation.rotate_vector(Vector3::unit_y());

    let target = position + forward;

    let view = Matrix4::look_at_rh(Point3::from_vec(position), Point3::from_vec(target), up);

    cgmath_to_mat4((projection * view).transpose())
}

fn cgmath_to_mat4(m: Matrix4<f32>) -> Mat4 {
    let cols = m.transpose(); // cgmath is column-major, you want column-major in [[f32; 4]; 4]
    let mut data = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            data[i][j] = cols[i][j];
        }
    }
    Mat4 { data }
}
