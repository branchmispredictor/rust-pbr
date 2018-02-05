use object::*;
use vector::*;

pub const DIST_EPSILON: f64 = 1e-4;

#[derive(Debug,Copy,Clone)]
pub struct Ray {
    pub point: Vector3,
    pub vector: Vector3,
}

impl Ray {
    pub fn new(point: Vector3, vector: Vector3) -> Ray {
        Ray {
            point: point,
            vector: vector.normalize(),
        }
    }
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub point: Vector3,
    pub normal: Vector3,
    pub object: &'a Visible,
}

impl<'a> Intersection<'a> {
    pub fn bounce(&self, ray: &Ray) -> Option<Ray> {
        None//Some(Ray::new(ray.point, self.normal))
    }
}
