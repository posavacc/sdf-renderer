use std::{iter::Sum, ops};
use rand::{RngExt, rng};
use rand_distr::StandardNormal;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    // self is the direction of the hemisphere
    // self is assumed normalized
    pub fn random_unit_hemisphere(self) -> Self {
        let rand = Vec3::random_unit_vec();
        if self.dot(rand) < 0.0 { -rand } else { rand }
    }

    /// uses normal distribution
    pub fn random_unit_vec() -> Self {
        let x = rng().sample::<f64, _>(StandardNormal) * 2.0 - 1.0;
        let y = rng().sample::<f64, _>(StandardNormal) * 2.0 - 1.0;
        let z = rng().sample::<f64, _>(StandardNormal) * 2.0 - 1.0;

        Self { x, y, z }
    }

    pub fn max_comp(self) -> f64 {
        self.x.max(self.y.max(self.z))
    }

    /// element-wise max
    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
            z: self.z.max(rhs.z)
        }
    }

    /// element-wise max
    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
            z: self.z.min(rhs.z)
        }
    }

    pub fn abs(self) -> Self {
        Self { x: self.x.abs(), y: self.y.abs(), z: self.z.abs() }
    }

    pub fn normalize(self) -> Self {
        self / self.len()
    }

    pub fn len_sqrd(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(self) -> f64 {
        self.len_sqrd().sqrt()
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub const ZERO: Self = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONE: Self = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec3::ZERO, |acc, v| acc + v)
    }
}
