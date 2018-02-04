use object::*;
use ray::*;

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
        for object in &self.objects {
            let intersection = object.intersect(&ray);
            if intersection.is_some() {
                return intersection;
            }
        }

        None
    }
}
