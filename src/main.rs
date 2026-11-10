use std::fs::File;
use std::io::Write;
use std::process::ExitCode;
use rayon::prelude::*;

use sdf_renderer::object::{Material, Object};
use sdf_renderer::vec3::Vec3;
use sdf_renderer::camera::Camera;
use sdf_renderer::shape::Shape;
use sdf_renderer::parse_color;

const WIDTH: i32 = 800;
const SPP: i32 = 20;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let path = if args.len() == 1 {
        eprintln!("No argument provided, falling back to \"/output.ppm\" file path");
        "output.ppm".to_string()
    } else if args.len() != 2 {
        eprintln!("Please enter only one argument (output image path, either absolute or relative to current directory)");
        return ExitCode::from(1);
    } else {
        args[1].clone()
    };

    let cam = Camera::new(Vec3::new(0.0, 0.0, 0.0), 80.0_f64.to_radians(), WIDTH);

    let plane = Shape::Box { size: Vec3::new(10.0, 0.1, 4.9) };
    let sphere = Shape::Sphere { radius: 1.0 };

    let gray = Material::Lambertian { color: Vec3::new(0.5, 0.5, 0.5) };
    let blue = Material::Lambertian { color: Vec3::new(0.0, 0.3, 1.0) };
    let red = Material::Lambertian { color: Vec3::new(1.0, 0.0, 0.0) };

    let o1 = Object::new(plane, Vec3::new(0.0, -1.1, -5.0), gray);
    let o2 = Object::new(sphere, Vec3::new(1.0, 0.0, -5.0), blue);
    let o3 = Object::new(sphere, Vec3::new(-1.0, 0.0, -5.0), red);

    let objects = vec![o1, o2, o3];

    let mut output_file = File::create(path).expect("should be able to open or create file");
    let ppm_header = format!("P3\n{} {}\n255\n", cam.img_width, cam.img_height);
    let mut image: String = String::with_capacity(ppm_header.capacity() + (cam.img_width * cam.img_height * 11) as usize);
    image.push_str(&ppm_header);

    let mut idcs = Vec::new();
    for line in 0..cam.img_height {
        for col in 0..cam.img_width {
            idcs.push(line * cam.img_width + col);
        }
    }

    let pixels: Vec<Vec3> = idcs.par_iter().map(|i| {
        let (line, col) = (i / cam.img_width, i % cam.img_width);
        let ray = cam.get_ray(line, col);
        let mut color = Vec3::ZERO;

        for _ in 0..SPP {
            color += ray.trace(&objects);
        }

        color / SPP as f64
    }).collect();

    let pixels: String = pixels.into_iter().map(|color| parse_color(color)).collect();

    image.push_str(&pixels);

    eprintln!("Writing to file...");
    output_file.write_all(image.as_bytes()).expect("should be able to write to file");

    eprintln!("Done");
    return ExitCode::from(0);
}
