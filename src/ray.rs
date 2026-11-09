use crate::vec3::Vec3;
use crate::shape::Shape;

const MARCH_DEPTH: i32 = 50;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Self { origin, dir }
    }

    pub fn march(self, shapes: &[Shape]) -> Vec3 {
        let mut closest = f64::INFINITY;
        let mut ray = self;

        for _ in 0..MARCH_DEPTH {
            for s in shapes {
                let dst = s.signed_distance(ray.origin);
                if dst < closest {
                    closest = dst
                }
            }

            if closest < 1e-4 {
                return Vec3::new(1.0, 1.0, 1.0);
            } else if closest > 1e4 {
                return Vec3::ZERO;
            }

            ray.origin = ray.at(closest);

            closest = f64::INFINITY;
        }

        return Vec3::ZERO;
    }

    fn at(self, t: f64) -> Vec3 {
        self.origin + self.dir * t
    }
}
