use crate::shape::Shape;
use crate::vec3::Vec3;

// TODO: add rotation
#[derive(Copy, Clone)]
pub struct Object {
    pub shape: Shape,
    pub pos: Vec3,
    pub material: Material,
}

impl Object {
    pub fn new(shape: Shape, pos: Vec3, material: Material) -> Self {
        Self { shape, pos, material }
    }
}

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { color: Vec3 },
}
