use crate::vec3::Vec3;

pub enum Shape {
    Sphere { pos: Vec3, radius: f64 },
    Box { pos: Vec3, size: Vec3 },
}

impl Shape {
    pub fn signed_distance(&self, p: Vec3) -> f64 {
        match self {
            Shape::Sphere { pos, radius } => {
                let p = p - *pos;
                (p - *pos).len() - radius
            }

            Shape::Box { pos, size } => {
                let p = p - *pos;
                let q = p.abs() - *size * 0.5;
                q.max(Vec3::ZERO).len() + q.max_comp().min(0.0)
            }
        }
    }
}
