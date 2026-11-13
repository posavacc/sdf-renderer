use std::f64::consts::PI;

use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub enum Shape {
    Sphere { radius: f64 },
    Box { size: Vec3 },
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Sphere { radius } => 4.0 * PI * radius * radius,
            Shape::Box { size } => {
                let (a, b, c) = (2.0 * size.x, 2.0 * size.y, 2.0 * size.z);
                2.0 * (a*b + a*c + b*c)
            }
        }
    }
}
