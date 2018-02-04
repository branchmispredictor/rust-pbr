use object::*;
use vector::*;

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
