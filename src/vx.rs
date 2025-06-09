#[derive(Copy, Clone)]
pub struct Vx {
    pub pos: [f32; 3],
    pub color: [f32; 3],
}

implement_vertex!(Vx, pos, color);

// #[macro_export]
// macro_rules! spread {
//     ($x:expr) => {
//         ($x[0], $x[1], $x[2])
//     };
// }

#[macro_export]
macro_rules! vx {
    // Empty: vx![]
    () => {
        vec![]
    };

    // Single vertex: vx![x, y => r, g, b]
    ($x:expr, $y:expr, $z:expr => $r:expr, $g:expr, $b:expr) => {
        Vx {
            pos: [$x as f32, $y as f32, $z as f32],
            color: [$r as f32, $g as f32, $b as f32],
        }
    };

    // multiple vertices: vx![x1, y1 => r1, g1, b1; x2, y2 => r2, g2, b2; ...]
    ($($x:expr, $y:expr, $z:expr => $r:expr, $g:expr, $b:expr);* $(;)?) => {
        vec![$(
            vx {
                pos: [$x as f32, $y as f32, $z as f32],
                color: [$r as f32, $g as f32, $b as f32],
            }
        ),*]
    };
}
