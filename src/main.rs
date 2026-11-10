use sdf_renderer::object::{Material, Object};
use sdf_renderer::vec3::Vec3;
use sdf_renderer::camera::Camera;
use sdf_renderer::shape::Shape;
use sdf_renderer::write_color;

const WIDTH: i32 = 1920;
const SPP: i32 = 200;

fn main() {
    let cam = Camera::new(Vec3::new(0.0, 0.0, 0.0), 80.0_f64.to_radians(), WIDTH);

    let plane = Shape::Box { size: Vec3::new(10.0, 0.1, 4.9) };
    let sphere = Shape::Sphere { radius: 1.0 };

    let gray = Material::Lambertian { color: Vec3::new(0.5, 0.5, 0.5) };
    let blue = Material::Lambertian { color: Vec3::new(0.0, 0.3, 1.0) };

    let o1 = Object::new(plane, Vec3::new(0.0, -1.1, -5.0), gray);
    let o2 = Object::new(sphere, Vec3::new(0.0, 0.0, -5.0), blue);

    let objects = vec![o1, o2];

    print!("P3\n{} {}\n255\n", cam.img_width, cam.img_height);
    for line in 0..cam.img_height {
        for col in 0..cam.img_width {
            let ray = cam.get_ray(line, col);
            let mut color = Vec3::ZERO;

            for _ in 0..SPP {
                color += ray.trace(&objects);
            }

            write_color(color / SPP as f64);
        }
    }
}
