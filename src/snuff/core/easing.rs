pub fn linear(t : f32) -> f32 {
    t
}

pub fn in_quad(t : f32) -> f32 {
    t * t
}

pub fn out_quad(t : f32) -> f32 {
    t * (2.0 - t)
}

pub fn in_out_quad(t : f32) -> f32 {
    if t < 0.5 { 2.0 * t * t } else { -1.0 + (4.0 - 2.0 * t) * t }
}

pub fn in_cubic(t : f32) -> f32 {
    t * t * t
}

pub fn out_cubic(t : f32) -> f32 {
    let t2 = t - 1.0;
    t2 * t2 * t2 + 1.0
}

pub fn in_out_cubic(t : f32) -> f32 {
    if t < 0.5 { 4.0 * t * t * t } else { (t - 1.0) * (2.0 * t - 2.0) * (2.0 * t - 2.0) + 1.0 }
}