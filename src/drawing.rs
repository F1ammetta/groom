use glium::{
    DrawParameters, IndexBuffer, Program, Surface, VertexBuffer, glutin::surface::WindowSurface,
    uniform, vertex::PerInstance,
};

use crate::{mat::Mat4, vx::Vx};

#[macro_export]
macro_rules! glsl {
    ($shader:literal) => {
        match fs::read_to_string("shaders/".to_owned() + $shader + ".glsl") {
            Ok(s) => s,
            Err(e) => panic!("Unable to read file: shaders/{:}.glsl: {:?}", $shader, e),
        }
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
            println!("Error drawing: {:?}", e);
        }
    };

    match target.finish() {
        Ok(_) => {}
        Err(e) => println!("Failed to draw: {:?}", e),
    };
}
