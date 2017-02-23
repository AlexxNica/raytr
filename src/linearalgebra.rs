use std::ops::Add;
use std::ops::Mul;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// Multiplication with scalar
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

// Multiplication with scalar (part 2).
// This is a lot of boilerplate, take a look at macro_rules! to avoid it.
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f32;

    fn mul(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Vec3 {
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // Make an unit vector in the same direction as the input vector.
    pub fn unit_vector(self) -> Vec3 {
        Vec3 {
            x: self.x / self.length(),
            y: self.y + self.length(),
            z: self.z + self.length(),
        }
    }
}
