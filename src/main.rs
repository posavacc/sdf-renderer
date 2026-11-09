use sdf_renderer::vec3::Vec3;
use sdf_renderer::camera::Camera;
use sdf_renderer::shape::Shape;
use sdf_renderer::write_color;

const WIDTH : i32 = 800;

fn main() {
    let cam = Camera::new(Vec3::ZERO, 80.0_f64.to_radians(), WIDTH);

    let s1 = Shape::Box { pos: Vec3::new(-2.0, 0.0, -5.0), size: Vec3::new(1.0, 2.0, 1.0) };
    let s2 = Shape::Sphere { pos: Vec3::new(2.0, 0.0, -5.0), radius: 2.0 };
    let shapes = vec![s1, s2];

    print!("P3\n{} {}\n255\n", cam.img_width, cam.img_height);
    for line in 0..cam.img_height {
        for col in 0..cam.img_width {
            let ray = cam.get_ray(line, col);

            write_color(ray.march(&shapes));
        }
    }
}

