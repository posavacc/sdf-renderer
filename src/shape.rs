use crate::vec3::Vec3;

const EPSILON: f64 = 1e-8;

#[derive(Copy, Clone)]
pub enum Shape {
    Sphere { radius: f64 },
    Box { size: Vec3 },
}

impl Shape {
    pub fn signed_distance(&self, pos: Vec3, p: Vec3) -> f64 {
        match self {
            Shape::Sphere { radius } => {
                let p = p - pos;
                p.len() - radius
            }

            Shape::Box { size } => {
                let p = p - pos;
                let q = p.abs() - *size;
                q.max(Vec3::ZERO).len() + q.max_comp().min(0.0)
            }
        }
    }

    pub fn normal(&self, pos: Vec3, p: Vec3) -> Vec3 {
        Vec3::new(
            self.signed_distance(pos, p + Vec3::new(EPSILON, 0.0, 0.0)) -
            self.signed_distance(pos, p - Vec3::new(EPSILON, 0.0, 0.0)),
            self.signed_distance(pos, p + Vec3::new(0.0, EPSILON, 0.0)) -
            self.signed_distance(pos, p - Vec3::new(0.0, EPSILON, 0.0)),
            self.signed_distance(pos, p + Vec3::new(0.0, 0.0, EPSILON)) -
            self.signed_distance(pos, p - Vec3::new(0.0, 0.0, EPSILON))
        ).normalize()
    }
}
