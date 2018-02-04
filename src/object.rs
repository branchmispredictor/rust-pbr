use vector::*;
use ray::*;

pub trait Visible {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn color(&self) -> Vector3;
}

pub struct Sphere {
    pub point: Vector3,
    pub radius: f64,
}

impl Visible for Sphere {
    fn color(&self) -> Vector3 {
        v3(1.0, 0.0, 0.0)
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let v_ray_to_center = self.point - ray.point;
        let dist_to_center = v_ray_to_center.dot(&v_ray_to_center);
        let dist_along_ray = v_ray_to_center.dot(&ray.vector);

        let discriminant = (self.radius * self.radius)
            - dist_to_center + (dist_along_ray * dist_along_ray);

        if discriminant < 0.0 {
            // No intersection
            return None;
        }

        let distance = dist_along_ray - discriminant.sqrt();

        // Use ray.vector.normalize() if not guarenteed to be a unit vector
        let intersect_point = ray.point + ray.vector * distance;

        // Normalize by dividing by our radius, do not need to calculate the vector length
        let normal = (intersect_point - self.point) / self.radius;

        Some(Intersection{
            distance: distance,
            point: intersect_point,
            normal: normal,
            object: self,
        })
    }
}
