use vector::*;
use ray::*;

pub trait Visible {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn color(&self) -> Vector3;
}

pub struct Sphere {
    pub point: Vector3,
    pub radius: f64,
    pub color: Vector3,
}

impl Visible for Sphere {
    fn color(&self) -> Vector3 {
        self.color
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let v_ray_to_center = self.point - ray.point;
        let dist_to_center_sq = v_ray_to_center.dot(&v_ray_to_center);
        let dist_along_ray = v_ray_to_center.dot(&ray.vector);

        let discriminant = (self.radius * self.radius)
            - dist_to_center_sq + (dist_along_ray * dist_along_ray);

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

pub struct Plane {
    pub point: Vector3,
    pub normal: Vector3,
    pub color: Vector3,
}

impl Visible for Plane {
    fn color(&self) -> Vector3 {
        self.color
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let normal_component = self.normal.dot(&ray.vector);

        if normal_component == 0.0 {
            return None;
        }

        let Np = self.normal.dot(&ray.point);

        let D = self.normal.dot(&self.point);

        let distance = (D - Np) / normal_component;

        let intersect_point =  ray.point + ray.vector * distance;

        Some(Intersection{
            distance: distance,
            point:  intersect_point,
            normal: self.normal,
            object: self,
        })
    }
}
