use sdf_renderer::vec3::Vec3;
use sdf_renderer::ray::Ray;

const WIDTH : i32 = 800;
const ASPECT_RATIO: f64 = 16.0 / 9.0;

const MARCH_DEPTH: i32 = 50;

struct Camera {
    pos: Vec3,

    first_pixel_pos: Vec3,
    pixel_delta_v: Vec3,
    pixel_delta_h: Vec3,

    v_fov: f64,

    aspect_ratio: f64,
    img_width: i32,
    img_height: i32,
}

impl Camera {
    fn new(pos: Vec3, fov: f64, img_width: i32) -> Self {
        let aspect_ratio = ASPECT_RATIO;
        let img_height = (img_width as f64 / aspect_ratio).round() as i32;

        let h = (fov / 2.0).tan();
        let l = h * aspect_ratio;

        let delta_v = (h * 2.0) / img_height as f64;
        let delta_h = (l * 2.0) / img_width as f64;

        let right = Vec3::new(1.0, 0.0, 0.0);
        let down = Vec3::new(0.0, -1.0, 0.0);

        let pixel_delta_v = delta_v * down;
        let pixel_delta_h = delta_h * right;

        let fpp = Vec3::new(pos.x - l + delta_h * 0.5, pos.y + h - delta_v * 0.5, pos.z - 1.0);

        Self {
            pos,
            first_pixel_pos: fpp,
            pixel_delta_v,
            pixel_delta_h,
            v_fov: fov,
            aspect_ratio,
            img_width,
            img_height,
        }
    }

    fn get_ray(&self, line: i32, col: i32) -> Ray {
        let origin = self.pos;
        let pixel_pos = self.first_pixel_pos + self.pixel_delta_h * col as f64 + self.pixel_delta_v * line as f64;
        let dir = pixel_pos - origin;

        Ray::new(origin, dir.normalize())
    }
}

fn main() {
    let cam = Camera::new(Vec3::ZERO, 60.0_f64.to_radians(), WIDTH);

    let s1 = Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0);
    let s2 = Sphere::new(Vec3::new(1.0, 1.0, -2.0), 0.3);
    let spheres = vec![s1, s2];

    print!("P3\n{} {}\n255\n", cam.img_width, cam.img_height);
    for line in 0..cam.img_height {
        for col in 0..cam.img_width {
            let ray = cam.get_ray(line, col);

            write_color(ray_march(ray, &spheres, col, line));
        }
    }
}

fn ray_march(first_ray: Ray, spheres: &[Sphere], col: i32, line: i32) -> Vec3 {
    let mut closest = f64::INFINITY;
    let mut ray = first_ray;

    for _ in 0..MARCH_DEPTH {
        for s in spheres {
            let dst = sphere_sdf(s, ray.origin - s.pos);
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

struct Sphere {
    pos: Vec3,
    radius: f64
}

impl Sphere {
    pub fn new(pos: Vec3, radius: f64) -> Self {
        Self { pos, radius }
    }
}

fn sphere_sdf(s: &Sphere, p: Vec3) -> f64 {
    p.len() - s.radius
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component < 0.0 {
        return 0.0;
    }

    return linear_component.sqrt();
}

fn write_color(color: Vec3) {
    let r = linear_to_gamma(color.x);
    let g = linear_to_gamma(color.y);
    let b = linear_to_gamma(color.z);

    let r = (256.0 * r.clamp(0.0, 0.999)) as i32;
    let g = (256.0 * g.clamp(0.0, 0.999)) as i32;
    let b = (256.0 * b.clamp(0.0, 0.999)) as i32;

    print!("{r} {g} {b}\n");
}
