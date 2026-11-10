use crate::vec3::Vec3;

pub mod vec3;
pub mod ray;
pub mod camera;
pub mod shape;
pub mod object;

pub fn parse_color(color: Vec3) -> String {
    let r = linear_to_gamma(color.x);
    let g = linear_to_gamma(color.y);
    let b = linear_to_gamma(color.z);

    let r = (256.0 * r.clamp(0.0, 0.999)) as i32;
    let g = (256.0 * g.clamp(0.0, 0.999)) as i32;
    let b = (256.0 * b.clamp(0.0, 0.999)) as i32;

    format!("{r} {g} {b}\n")
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 { linear_component.sqrt() } else { 0.0 }
}
