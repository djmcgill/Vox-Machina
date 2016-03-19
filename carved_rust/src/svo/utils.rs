use nalgebra::Vec3;

pub fn sorted_ts(ray_origin: f32, inv_ray_dir: f32) -> (f32, f32) {
    let t1 = (0.-ray_origin) * inv_ray_dir;
    let t2 = (1.-ray_origin) * inv_ray_dir;
    if t1 < t2 { (t1, t2) } else { (t2, t1) }
}

// TODO: test these specifically
pub fn to_child_space(vec: Vec3<f32>, offsets: Vec3<f32>) -> Vec3<f32> {
    (vec - offsets*0.5)*2.
}

// TODO: test these specifically
pub fn from_child_space(vec: Vec3<f32>, offsets: Vec3<f32>) -> Vec3<f32> {
    vec*0.5 + offsets*0.5
}

// Returns a vector with either 0. or 1. as its elements
fn above_axis(ix: u8) -> Vec3<f32> {
    Vec3::new((ix & 1) as f32,
              ((ix >> 1) & 1) as f32,
              ((ix >> 2) & 1) as f32)
}


// Returns the new origin of the child at the given index in global space.
pub fn offset(ix: u8, depth: i32) -> Vec3<f32> {
    above_axis(ix) / ((1 << depth+1) as f32)
}