use crate::vx;

#[macro_export]
macro_rules! rect {
    ($x:expr, $y:expr, $w:expr, $h:expr, $r:expr, $g:expr, $b:expr) => {{
        let p0 = vx!($x, $y, 0.0 => $r, $g, $b);
        let p1 = vx!($x + $w, $y ,0.0 => $r, $g, $b);
        let p2 = vx!($x + $w, $y + $h, 0.0=> $r, $g, $b);
        let p3 = vx!($x, $y + $h, 0.0=> $r, $g, $b);
        vec![p0, p1, p2, p0, p2, p3]
    }};
}
