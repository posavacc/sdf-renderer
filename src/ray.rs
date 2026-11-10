use crate::object::{Material, Object};
use crate::vec3::Vec3;

const BOUNCE_LIMIT: i32 = 30;

const MAX_STEPS: i32 = 128;
const MAX_DIST: f64 = 1e4;
const EPSILON: f64 = 1e-8;

const SKY_COLOR: Vec3 = Vec3 { x: 0.5, y: 0.7, z: 1.0 };

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Self { origin, dir }
    }

    pub fn trace(self, objects: &[Object]) -> Vec3 {
        let mut ray_color = Vec3::new(1.0, 1.0, 1.0);
        let mut ray = self;

        for _ in 0..=BOUNCE_LIMIT {
            if let Some(hit_info) = ray.march(objects) {
                match hit_info.material {
                    Material::Lambertian { color } => {
                        ray.origin = hit_info.point + hit_info.normal * 1e-6;
                        ray.dir = lambertian_scatter(ray, hit_info.normal, 1.0);

                        ray_color *= color;
                    }
                }
            } else {
                return ray_color * sky_color(ray.dir);
            }
        }

        return Vec3::ZERO;
    }

    fn march(self, objects: &[Object]) -> Option<HitInfo> {
        let mut closest_dst = f64::INFINITY;
        let mut closest_obj: Option<Object> = None;
        let mut ray = self;


        for _ in 0..MAX_STEPS {
            for obj in objects {
                let dst = obj.shape.signed_distance(obj.pos, ray.origin);
                if dst < closest_dst {
                    closest_dst = dst;
                    closest_obj = Some(*obj);
                }
            }

            if let Some(obj) = closest_obj {
                if closest_dst < EPSILON {
                    let info = HitInfo::new(
                        ray.at(closest_dst),
                        obj.shape.normal(obj.pos, ray.origin),
                        obj.material
                    );
                    return Some(info);
                }
            }

            if closest_dst > MAX_DIST {
                return None;
            }

            ray.origin = ray.at(closest_dst);

            closest_dst = f64::INFINITY;
        }

        if let Some(obj) = closest_obj {
            let info = HitInfo::new(
                ray.origin,
                obj.shape.normal(obj.pos, ray.origin),
                obj.material
            );
            return Some(info);
        } else {
            return None;
        }
    }

    fn at(self, t: f64) -> Vec3 {
        self.origin + self.dir * t
    }
}

fn random_unit_hemisphere(n: Vec3) -> Vec3 {
    let mut result = Vec3::random();
    while result.len_sqrd() > 1.0 {
        result = Vec3::random();
    }
    result = result.normalize();

    return if n.dot(result) < 0.0 { -result } else { result };
}

fn lambertian_scatter(ray: Ray, normal: Vec3, smoothness: f64) -> Vec3 {
    random_unit_hemisphere(normal) + normal
}

fn sky_color(dir: Vec3) -> Vec3 {
    let a = 0.5 * (dir.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + SKY_COLOR * a
}

struct HitInfo {
    point: Vec3,
    normal: Vec3,
    material: Material,
}

impl HitInfo {
    fn new(point: Vec3, normal: Vec3, material: Material) -> Self {
        Self { point, normal, material }
    }
}
