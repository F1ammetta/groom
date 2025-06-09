use glium::uniforms::{AsUniformValue, UniformValue};

#[derive(Debug, Copy, Clone)]
pub struct Mat4 {
    pub data: [[f32; 4]; 4], // column-major
}

impl AsUniformValue for Mat4 {
    fn as_uniform_value(&self) -> UniformValue {
        UniformValue::Mat4(self.data)
    }
}

#[macro_export]
macro_rules! mat {
    [$($($val:expr),+);+ $(;)?] => {{
        let rows: Vec<Vec<f32>> = vec![$(vec![$($val as f32),+]),+];

        let mut data = [[0.0; 4]; 4]; // column-major

        for i in 0..4 {
            for j in 0..4 {
                data[i][j] = rows[j][i]; // transpose row-major input to column-major
            }
        }

        Mat4 { data }
    }};
}
