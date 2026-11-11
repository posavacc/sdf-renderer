use crate::Vec3;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { color: Vec3, smoothness: f64 },
    LightSource { color: Vec3, strength: f64 },
}

impl Material {
    pub fn lambertian_scatter(in_dir: Vec3, n: Vec3, smoothness: f64) -> Vec3 {
        let lamb_scatter = (Vec3::random_unit_vec() + n).normalize();
        let specular_reflection = reflect(in_dir, n);

        lerp(lamb_scatter, specular_reflection, smoothness).normalize()
    }
}

fn reflect(in_dir: Vec3, n: Vec3) -> Vec3 {
    let para = in_dir.dot(n) * n;
    let perp = in_dir - para;
    return perp - para;
}

fn lerp(a: Vec3, b: Vec3, t: f64) -> Vec3 {
    a + t * (b - a)
}
