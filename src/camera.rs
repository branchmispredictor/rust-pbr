use ray::*;
use vector::*;

pub struct Camera {
    // Intrinsic values
    pub width: u32,
    pub height: u32,
    pub point: Vector3,
    pub vector: Vector3,

    // Computed values
    pub v_right: Vector3,
    pub v_up: Vector3,
    pub height_width_ratio: f64,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Camera {
        Camera {
            width: width,
            height: height,
            point: V3_ZERO,
            vector: V3_IN,
            v_right: V3_RIGHT,
            v_up: V3_UP,
            height_width_ratio: height as f64 / width as f64,
        }
    }

    pub fn ray_at(&self, u: f64, v: f64) -> Ray {
        let xcomp = self.v_right * u;
        let ycomp = self.v_up * v * self.height_width_ratio;

        Ray::new(self.point, self.vector + xcomp + ycomp)
    }

    pub fn look_at(&mut self, point: Vector3) {
        self.vector = (point - self.point).normalize();

        // Calculate Right and Up normals
        self.v_right = self.vector.cross(&V3_UP).normalize();
        self.v_up = self.v_right.cross(&self.vector).normalize();
    }

    pub fn move_to(&mut self, point: Vector3) {
        self.point = point;
    }
}
