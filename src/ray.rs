use std::f64::consts::PI;

use rand::{RngExt, rng};

use crate::object::Object;
use crate::material::Material;
use crate::vec3::Vec3;

const NIGHT: bool = true;

const BOUNCE_LIMIT: i32 = 30;

const MAX_STEPS: i32 = 256;
const MAX_DIST: f64 = 1e3;
const EPSILON: f64 = 1e-4;
const BOUNCE_OFFSET: f64 = 1e-3;

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

    pub fn trace(self, objects: &[Object], maybe_lights: Option<&[Object]>) -> Vec3 {
        let mut throughput = Vec3::ONE;
        let mut radiance = Vec3::ZERO;
        let mut ray = self;

        for _ in 0..=BOUNCE_LIMIT {
            if let Some(hit_info) = ray.march(objects) {
                let obj = objects[hit_info.idx];
                match obj.material {
                    Material::Lambertian { color, smoothness } => {
                        ray.origin = hit_info.point + hit_info.normal * BOUNCE_OFFSET;

                        ray.dir = Material::lambertian_scatter(ray.dir, hit_info.normal, smoothness);

                        let nee_contribution = ray.shadow_ray(hit_info.normal, objects, maybe_lights);

                        throughput *= color;
                        radiance += throughput * nee_contribution;
                    },
                    Material::LightSource { color, strength } => {
                        return throughput * color * strength + radiance;
                    }
                }
            } else {
                return throughput * sky_color(ray.dir) + radiance;
            }
        }

        return Vec3::ZERO + radiance;
    }

    fn march(self, objects: &[Object]) -> Option<HitInfo> {
        let mut closest_dst = f64::INFINITY;
        let mut closest_obj: Option<usize> = None;
        let mut ray = self;
        let mut t = 0.0;

        for _ in 0..MAX_STEPS {
            for (i, obj) in objects.iter().enumerate() {
                let dst = obj.signed_distance(ray.origin);
                if dst < closest_dst {
                    closest_dst = dst;
                    closest_obj = Some(i);
                }
            }

            if let Some(obj_idx) = closest_obj {
                if closest_dst < EPSILON * t {
                    let obj = objects[obj_idx];
                    let info = HitInfo::new(
                        ray.at(closest_dst),
                        obj.normal(ray.origin),
                        obj_idx
                    );
                    return Some(info);
                }
            }

            if closest_dst > MAX_DIST {
                return None;
            }

            ray.origin = ray.at(closest_dst);

            t += closest_dst;

            closest_dst = f64::INFINITY;
        }

        if let Some(obj_idx) = closest_obj {
            let obj = objects[obj_idx];
            let info = HitInfo::new(
                ray.at(closest_dst),
                obj.normal(ray.origin),
                obj_idx
            );
            return Some(info);
        } else {
            return None;
        }
    }

    /// returns the amount of light that a given lightsource contributes
    fn shadow_ray(&self,
        normal: Vec3,
        all_objects: &[Object],
        maybe_lights: Option<&[Object]>
    ) -> Vec3 {
        if let Some(lights) = maybe_lights {
            let light = lights[rng().random_range(0..lights.len())];
            let surface_point = light.sample_surface();

            let displacement = surface_point - self.origin;
            let dst = displacement.len();
            let dir = displacement / dst;
            let ray = Ray::new(self.origin, dir);

            if normal.dot(dir) < 0.0 { return Vec3::ZERO; }

            let light_normal = light.normal(surface_point);
            if light_normal.dot(-dir) < 0.0 { return Vec3::ZERO; }

            if let Some(hit_info) = ray.march(all_objects) {
                let did_hit_light = (hit_info.point - surface_point).len_sqrd() <= EPSILON * EPSILON;
                let did_hit_light = hit_info.idx == all_objects.iter().position(|o| *o == light);
                if did_hit_light {
                    let area_pdf = 1.0 / light.shape.area();

                    // between light_normal and -dir
                    let cos_theta = light_normal.dot(-dir);

                    let pdf = area_pdf * dst * dst / cos_theta;

                    let contribution = match light.material {
                        Material::LightSource { color, strength } => color * strength,
                        _ => panic!("light list contains non-light material")
                    };

                    // between surface normal and dir
                    let cos_theta = normal.dot(dir);

                    return (contribution * cos_theta) / (pdf * lights.len() as f64 * PI);
                }

                return Vec3::ZERO;
            }

            return Vec3::ZERO;
        }

        return Vec3::ZERO
    }

    fn at(self, t: f64) -> Vec3 {
        self.origin + self.dir * t
    }
}

fn sky_color(dir: Vec3) -> Vec3 {
    if NIGHT {
        Vec3::ZERO
    } else {
        let a = 0.5 * (dir.y + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + SKY_COLOR * a
    }
}

struct HitInfo {
    point: Vec3,
    normal: Vec3,
    idx: usize,
}

impl HitInfo {
    fn new(point: Vec3, normal: Vec3, idx: usize) -> Self {
        Self { point, normal, idx}
    }
}
