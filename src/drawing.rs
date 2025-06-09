use glium::{
    DrawParameters, IndexBuffer, Program, Surface, VertexBuffer, glutin::surface::WindowSurface,
    vertex::PerInstance,
};

use crate::{mat::Mat4, vx::Vx};

#[macro_export]
macro_rules! glsl {
    ($shader:literal) => {
        fs::read_to_string("shaders/".to_owned() + $shader + ".glsl").unwrap()
    };
}

pub fn draw_shape(
    display: &glium::Display<WindowSurface>,
    indices: &IndexBuffer<u16>,
    program: &Program,
    vi_buf: (&VertexBuffer<Vx>, PerInstance),
    matrix: &Mat4,
    params: &DrawParameters,
) {
    let mut target = display.draw();

    target.clear_color_and_depth((0.0 / 255.0, 120.0 / 255.0, 140.0 / 255.0, 1.0), 1.0);

    // let instances = instance_buffer.per_instance().unwrap();

    match target.draw(
        vi_buf,
        indices,
        program,
        &uniform! {
            matrix: *matrix
        },
        params,
    ) {
        Ok(_) => {}
        Err(e) => {
            dbg!(e);
        }
    };

    target.finish().unwrap();
}
