use sdf_renderer::vec3::Vec3;

const WIDTH : i32 = 100;
const HEIGHT: i32 = 100;

fn main() {

    print!("P3\n{} {}\n255\n", WIDTH, HEIGHT);
    for line in 0..HEIGHT {
        for col in 0..WIDTH {

        }
    }
}

fn sphere_sdf(point: Vec3, radius: f64) -> f64 {
    point.len() - radius
}
