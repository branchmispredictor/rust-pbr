use object::*;
use ray::*;

use std::cmp::Ordering::Equal;

pub struct Scene<'a> {
    objects: Vec<Box<Visible + 'a>>
}

impl<'a> Scene<'a> {
    pub fn new() -> Scene<'a> {
        Scene {
            objects: Vec::new(),
        }
    }

    pub fn add<T: Visible + 'a>(&mut self, o: T) {
        self.objects.push(Box::new(o));
    }

    pub fn intersect(&self, ray: Ray) -> Option<Intersection> {
        self.objects.iter()
            .filter_map(|o| o.intersect(&ray))
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap_or(Equal))
    }
}
