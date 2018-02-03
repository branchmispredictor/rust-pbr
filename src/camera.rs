use vector::*;

pub struct Camera {
    // Intrinsic values
    width: u32,
    height: u32,
    pub point: Vector3,
    pub fovRadians: f64,
    pub vector: Vector3,

    // Computed values
    pub v_right: Vector3,
    pub v_up: Vector3,
}

impl Camera {
    pub fn new(width: u32, height: u32, fieldOfView: f64) -> Camera {
        let fovRadians = ::std::f64::consts::PI * fieldOfView / 360.0;
        //let heightWidthRatio = height as f64 / width as f64;
        //let halfWidth = fovRadians.tan();
        //let halfHeight = heightWidthRatio * halfWidth;

        Camera {
            width: width,
            height: height,
            point: V3_ZERO,
            fovRadians: fovRadians,
            vector: V3_IN,
            v_right: V3_RIGHT,
            v_up: V3_UP,
        }
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
