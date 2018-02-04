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
        let mut closest: Option<Intersection> = None;

        for object in &self.objects {
            if let Some(intersection) = object.intersect(&ray) {
                if closest.as_ref().is_none() || intersection.distance < closest.as_ref().unwrap().distance {
                    closest = Some(intersection);
                }
            }
        }

        closest
    }
}
