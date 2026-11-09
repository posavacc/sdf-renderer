use crate::vec3::Vec3;
use crate::ray::Ray;

const ASPECT_RATIO: f64 = 16.0 / 9.0;

pub struct Camera {
    pos: Vec3,

    first_pixel_pos: Vec3,
    pixel_delta_v: Vec3,
    pixel_delta_h: Vec3,

    pub img_width: i32,
    pub img_height: i32,
}

impl Camera {
    pub fn new(pos: Vec3, fov: f64, img_width: i32) -> Self {
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
            img_width,
            img_height,
        }
    }

    pub fn get_ray(&self, line: i32, col: i32) -> Ray {
        let origin = self.pos;
        let pixel_pos = self.first_pixel_pos + self.pixel_delta_h * col as f64 + self.pixel_delta_v * line as f64;
        let dir = pixel_pos - origin;

        Ray::new(origin, dir.normalize())
    }
}
