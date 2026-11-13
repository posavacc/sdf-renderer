use crate::shape::Shape;
use crate::vec3::Vec3;
use crate::material::Material;

const EPSILON: f64 = 1e-4;

// TODO: add rotation
#[derive(Copy, Clone, PartialEq)]
pub struct Object {
    pub shape: Shape,
    pub pos: Vec3,
    pub material: Material,
}

impl Object {
    pub fn new(shape: Shape, pos: Vec3, material: Material) -> Self {
        Self { shape, pos, material }
    }

    pub fn signed_distance(&self, p: Vec3) -> f64 {
        match self.shape {
            Shape::Sphere { radius } => {
                let p = p - self.pos;
                p.len() - radius
            }

            Shape::Box { size } => {
                let p = p - self.pos;
                let q = p.abs() - size;
                q.max(Vec3::ZERO).len() + q.max_comp().min(0.0)
            }
        }
    }

    pub fn normal(&self, p: Vec3) -> Vec3 {
        match self.shape {
            Shape::Sphere { radius: _ } => (p - self.pos).normalize(),
            _ => (
                Vec3::new( 1.0, -1.0, -1.0) * self.signed_distance(p + Vec3::new( 1.0, -1.0, -1.0) * EPSILON) +
                Vec3::new(-1.0, -1.0,  1.0) * self.signed_distance(p + Vec3::new(-1.0, -1.0,  1.0) * EPSILON) +
                Vec3::new(-1.0,  1.0, -1.0) * self.signed_distance(p + Vec3::new(-1.0,  1.0, -1.0) * EPSILON) +
                Vec3::new( 1.0,  1.0,  1.0) * self.signed_distance(p + Vec3::new( 1.0,  1.0,  1.0) * EPSILON)
            ).normalize()
        }
    }

    /// Returns a random point on the surface of the shape
    pub fn sample_surface(&self) -> Vec3 {
        match self.shape {
            Shape::Sphere { radius } => {
                self.pos + Vec3::random_unit_vec() * radius
            },
            Shape::Box { size: _ } => {
                todo!()
            }
        }
    }
}
