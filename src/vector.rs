use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;

#[derive(Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3 {
    pub fn length(&self) -> f64 {
        let &Vector3 {x, y, z} = self;
        (x * x + y * y + z * z).sqrt()
    }

    pub fn normalize(&self) -> Vector3 {
        self / self.length()
    }
}

pub fn v3(x: f64, y: f64, z: f64) -> Vector3 {
    Vector3 {x, y, z}
}

impl<'a, 'b> Add<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn add(self, other: &'b Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a> Add<f64> for &'a Vector3 {
    type Output = Vector3;

    fn add(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl<'a> Mul<f64> for &'a Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<'a> Div<f64> for &'a Vector3 {
    type Output = Vector3;

    fn div(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

pub struct Vector4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

pub fn v4(x: f64, y: f64, z: f64, w: f64) -> Vector4 {
    Vector4 {x, y, z, w}
}

impl Vector4 {
    fn length(&self) -> f64 {
        let &Vector4 {x, y, z, w} = self;
        (x * x + y * y + z * z + w * w).sqrt()
    }

    fn normalize(&self) -> Vector4 {
        let &Vector4 {x, y, z, w} = self;
        let len = self.length();
        v4(x / len, y / len, z / len, w / len)
    }
}
