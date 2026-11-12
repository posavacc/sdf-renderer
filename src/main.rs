use std::fs::File;
use std::io::Write;
use std::process::ExitCode;
use std::sync::atomic::{self, AtomicU64};
use rayon::prelude::*;

use sdf_renderer::object::Object;
use sdf_renderer::material::Material;
use sdf_renderer::vec3::Vec3;
use sdf_renderer::camera::Camera;
use sdf_renderer::shape::Shape;
use sdf_renderer::parse_color;

const WIDTH: i32 = 400;
const SPP: i32 = 512;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let path = if args.len() == 1 {
        eprintln!("No argument provided, falling back to \"output.ppm\" file path");
        "output.ppm".to_string()
    } else if args.len() != 2 {
        eprintln!("Please enter only one argument (output image path, either absolute or relative to current directory)");
        return ExitCode::from(1);
    } else {
        args[1].clone()
    };

    let cam = Camera::new(Vec3::new(0.0, 0.0, 5.0), 50.0_f64.to_radians(), WIDTH);

    let plane = Shape::Box { size: Vec3::new(2.0, 0.1, 2.0) };
    let side_wall = Shape::Box { size: Vec3::new(0.1, 2.5, 2.0) };
    let back_wall = Shape::Box { size: Vec3::new(2.0, 2.5, 0.1) };
    let led = Shape::Box { size: Vec3::new(0.4, 0.1, 0.2) };
    let sphere = Shape::Sphere { radius: 0.5 };

    let red = Material::Lambertian { color: Vec3::new(1.0, 0.0, 0.0), smoothness: 0.0 };
    let green = Material::Lambertian { color: Vec3::new(0.0, 1.0, 0.0), smoothness: 0.0 };
    let gray = Material::Lambertian { color: Vec3::new(0.9, 0.9, 0.9), smoothness: 0.0 };
    let s0 = Material::Lambertian { color: Vec3::ONE, smoothness: 0.0 };
    let s1 = Material::Lambertian { color: Vec3::ONE, smoothness: 0.333333};
    let s2 = Material::Lambertian { color: Vec3::ONE, smoothness: 0.666666 };
    let s3 = Material::Lambertian { color: Vec3::ONE, smoothness: 1.0 };
    let light = Material::LightSource { color: Vec3::ONE, strength: 5.0 };

    let floor = Object::new(plane, Vec3::new(0.0, -2.6, -5.0), gray);
    let ceiling = Object::new(plane, Vec3::new(0.0, 2.6, -5.0), gray);
    let left_wall = Object::new(side_wall, Vec3::new(-2.1, 0.0, -5.0), green);
    let right_wall = Object::new(side_wall, Vec3::new(2.1, 0.0, -5.0), red);
    let back_wall = Object::new(back_wall, Vec3::new(0.0, 0.0, -7.1), gray);
    let led = Object::new(led, Vec3::new(0.0, 2.5, -5.0), light);

    let s0 = Object::new(sphere, Vec3::new(-1.5, -2.0, -4.0), s0);
    let s1 = Object::new(sphere, Vec3::new(-0.5, -2.0, -5.0), s1);
    let s2 = Object::new(sphere, Vec3::new(0.5, -2.0, -4.0), s2);
    let s3 = Object::new(sphere, Vec3::new(1.5, -2.0, -5.0), s3);

    let objects = vec![s0, s1, s2, s3, /*led,*/ floor, ceiling, left_wall, right_wall, back_wall];

    let mut output_file = File::create(path).expect("should be able to open or create file");
    let ppm_header = format!("P3\n{} {}\n255\n", cam.img_width, cam.img_height);
    let mut image: String = String::with_capacity(ppm_header.capacity() + (cam.img_width * cam.img_height * 11) as usize);
    image.push_str(&ppm_header);

    let counter: atomic::AtomicU64 = AtomicU64::new(0);
    let pixels: Vec<Vec3> = (0..(cam.img_width * cam.img_height)).into_par_iter().map(|i| {
        let (line, col) = (i / cam.img_width, i % cam.img_width);

        let pixels_iterated = counter.load(atomic::Ordering::Relaxed);

        if pixels_iterated % 1000 == 0 {
            let ratio = pixels_iterated as f64 / (cam.img_width * cam.img_height) as f64;
            eprint!("\r{:.2}%", ratio * 100.0);
        }

        let ray = cam.get_ray(line, col);

        _ = counter.fetch_add(1, atomic::Ordering::Relaxed);

        (0..SPP).map(|_| ray.trace(&objects)).sum::<Vec3>() / SPP as f64
    }).collect();

    eprint!("\r      ");
    eprint!("\r100%\n");

    let pixels: String = pixels.into_iter().map(|color| parse_color(color)).collect();

    image.push_str(&pixels);

    eprintln!("Writing to file...");
    output_file.write_all(image.as_bytes()).expect("should be able to write to file");

    eprintln!("Done");
    return ExitCode::from(0);
}
