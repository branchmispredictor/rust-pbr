use material::*;
use ray::*;
use vector::*;

pub trait Visible {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn material(&self) -> &Material;
}

pub struct Sphere {
    pub point: Vector3,
    pub radius: f64,
    pub material: Material,
}

impl Visible for Sphere {
    fn material(&self) -> &Material {
        &self.material
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

        let disc_sq = discriminant.sqrt();

        let mut distance = 0.0;
        if dist_along_ray - disc_sq > 0.0005 {
            distance = dist_along_ray - disc_sq;
        } else if dist_along_ray + disc_sq > 0.0005 {
            distance = dist_along_ray + disc_sq
        } else {
            return None;
        }

        // Use ray.vector.normalize() if not guarenteed to be a unit vector
        let intersect_point = ray.point + ray.vector * distance;

        // Normalize by dividing by our radius, do not need to calculate the vector length
        let mut normal = (intersect_point - self.point) / self.radius;

        // Ensure it is the front-facing normal
        if normal.dot(&ray.vector) > 0.0 {
            normal = normal * -1.0;
        }

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
    pub material: Material,
}

impl Visible for Plane {
    fn material(&self) -> &Material {
        &self.material
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

        let mut normal = self.normal;

        // Ensure it is the front-facing normal
        if normal.dot(&ray.vector) < 0.0 {
            normal = normal * -1.0;
        }

        Some(Intersection{
            distance: distance,
            point:  intersect_point,
            normal: normal,
            object: self,
        })
    }
}
