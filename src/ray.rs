use shape::*;
use vector::*;

pub struct Ray {
    point: Vector3,
    vector: Vector3,
}

impl Ray {
    pub fn new(point: Vector3, vector: Vector3) -> Ray {
        Ray {
            point: point,
            vector: vector.normalize(),
        }
    }

    pub fn hit_sphere(&self, sphere: &Sphere) -> bool {
        let v_ray_to_center = sphere.point - self.point;
        let dist_along_ray = v_ray_to_center.dot(&self.vector);
        let dist_to_center = v_ray_to_center.dot(&v_ray_to_center);

        let discriminant = (sphere.radius * sphere.radius)
            - dist_to_center + (dist_along_ray * dist_along_ray);

        discriminant > 0.0
    }

    pub fn color(&self) -> Vector3 {
        let sphere = Sphere{point: v3(0.0, 0.0, 1.0), radius: 0.5};
        let sphere2 = Sphere{point: v3(2.0, 0.0, 2.0), radius: 0.5};
        let sphere3 = Sphere{point: v3(-2.0, 0.0, 2.0), radius: 0.5};

        if self.hit_sphere(&sphere) || self.hit_sphere(&sphere2) || self.hit_sphere(&sphere3) {
            return v3(1.0, 0.0, 0.0);
        }

        let t = 0.5 * (self.vector.y + 1.0);
        let white = &v3(1.0, 1.0, 1.0) * (1.0 - t);
        let blue = &v3(0.5, 0.7, 1.0) * t;

        return &white + &blue;
    }
}
