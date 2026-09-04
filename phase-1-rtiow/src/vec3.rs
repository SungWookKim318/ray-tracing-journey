#![allow(dead_code)]
use rand::RngExt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Point3 = Vec3;

// Constructor
impl Vec3 {
    pub const fn new(new_x: f32, new_y: f32, new_z: f32) -> Self {
        Self {
            x: new_x,
            y: new_y,
            z: new_z,
        }
    }

    pub const fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub const fn with_scalar(v: f32) -> Self {
        Self { x: v, y: v, z: v }
    }

    pub fn random(rng: &mut dyn rand::Rng) -> Self {
        Self {
            x: rng.random_range(0.0..=1.0),
            y: rng.random_range(0.0..=1.0),
            z: rng.random_range(0.0..=1.0),
        }
    }

    pub fn random_range(rng: &mut dyn rand::Rng, min: f32, max: f32) -> Self {
        Self {
            x: rng.random_range(min..max),
            y: rng.random_range(min..max),
            z: rng.random_range(min..max),
        }
    }

    pub fn random_sphere(rng: &mut dyn rand::Rng) -> Self {
        loop {
            let new_vec = Vec3::random(rng);
            let squre_len = new_vec.length_squared();

            if 1e-160 < squre_len || squre_len <= 1.0 {
                return new_vec.normalize();
            }
        }
    }

    pub fn random_hemisphere(rng: &mut dyn rand::Rng, normal: Vec3) -> Self {
        let random_sphere = Vec3::random_sphere(rng);

        if random_sphere.dot(normal) > 0.0 {
            random_sphere
        } else {
            -random_sphere
        }
    }

    pub fn random_unit_disk(rng: &mut dyn rand::Rng) -> Self {
        loop {
            let new_vec = Vec3::new(
                rng.random_range(-1.0..=1.0),
                rng.random_range(-1.0..=1.0),
                0.0,
            );
            if new_vec.length_squared() < 1.0 {
                return new_vec;
            }
        }
    }
}

impl Vec3 {
    pub fn is_near_zero(&self) -> bool {
        let epsilon = 0.00001f32;
        self.x.abs() < epsilon && self.y.abs() < epsilon && self.z.abs() < epsilon
    }

    pub fn refract(&self, normal_vector: Vec3, eta_i_over_eta_t: f32) -> Vec3 {
        let incident_direction = *self;
        let incident_cos = incident_direction.dot(-normal_vector).min(1.0);
        // R_perp + |R| * |n| * cos(theta) * n <= 벡터의 내적을 활용
        // R'_perp = (eta / eta') * R_perp <= 스넬의 법칙
        let refracted_perp = eta_i_over_eta_t * (incident_direction + incident_cos * normal_vector);
        // 이미 구한 R'_perp을 활용해서 피타고라스의 정리를 응용
        // |R'|^2 = |R'_perp|^2 + |R'_parallel|^2
        // 1 = |R'_perp|^2 + |R'_parallel|^2
        let refracted_parallel =
            -(1.0 - refracted_perp.length_squared()).abs().sqrt() * normal_vector;
        refracted_perp + refracted_parallel
    }
}

use std::ops::{Index, IndexMut};
impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds"),
        }
    }
}
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

// Operators
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}
impl Div<Vec3> for f32 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self / rhs.x, self / rhs.y, self / rhs.z)
    }
}
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

// Helper Calculation Functions

impl Vec3 {
    pub fn dot(self, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        self / self.length()
    }

    pub fn reflect(self, normal: Vec3) -> Self {
        self - 2.0 * self.dot(normal) * normal
    }
}
