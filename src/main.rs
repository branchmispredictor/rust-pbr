#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate pbr;
extern crate rand;

use pbr::*;

use std::error::Error;
use rand::Rng;

pub fn get_light(ray: &Ray, scene: &Scene, depth: u32, rng: &mut rand::ThreadRng) -> Vector3 {
    if let Some(intersection) = scene.intersect(ray) {
        let material = &intersection.object.material();
        let color = material.emission;

        if depth > 3 {
            return color;
        }

        // Pick a random new direction to bounce
        // http://raytracey.blogspot.com/2016/11/opencl-path-tracing-tutorial-2-path.html

        // Random numbers for random bounce
        let rand1 = 2.0 * std::f64::consts::PI * rng.gen::<f64>();
        let rand2 = rng.gen::<f64>();
        let rand2s = rand2.sqrt();

        // Split hit normal to coordinate frame
        let w = intersection.normal;
        let axis = if w.x.abs() > 0.1 {
            v3(0.0, 1.0, 0.0)
        } else {
            v3(1.0, 0.0, 0.0)
        };
        let u = axis.cross(&w).normalize();
        let v = w.cross(&u);

        // Generate new ray dir with random numbers
        let ray_dir = u*rand1.cos()*rand2s + v*rand1.sin()*rand2s + w*(1.0 - rand2).sqrt();
        let new_ray = Ray::new(intersection.point + intersection.normal * 0.00003, ray_dir);


        // Next color is cosine-weighted importance sampling for difffuse
        // Times color of the current hit
        //let weight = ray_dir.dot(&intersection.normal);
        return color + &(material.color/* * weight*/) * &get_light(&new_ray, scene, depth+1, rng);
    }

    let t = 0.5 * (ray.vector.y + 1.0);
    let white = v3(1.0, 1.0, 1.0) * (1.0 - t);
    let blue = v3(0.5, 0.7, 1.0) * t;

    V3_ZERO//white + blue
}

fn main() {
    let width = 640;
    let height = 480;

    let n_samples = 128;

    let mut camera = Camera::new(width, height);

    let mut rng = rand::thread_rng();

    camera.move_to(v3(0.0, 0.1, 2.0));
    camera.look_at(v3(0.0, 0.1, 0.0));

    let mut scene = Scene::new();

    let left_wall = Sphere{
        point: v3(-200.6, 0.0, 0.0),
        radius: 200.0,
        material: Material{
            color: v3(0.75, 0.25, 0.25),
            emission: v3(0.0, 0.0, 0.0),
        }
    };

    let right_wall = Sphere{
        point: v3(200.6, 0.0, 0.0),
        radius: 200.0,
        material: Material{
            color: v3(0.25, 0.25, 0.75),
            emission: v3(0.0, 0.0, 0.0),
        }
    };

    let floor = Sphere{
        point: v3(0.0, -200.4, 0.0),
        radius: 200.0,
        material: Material{
            color: v3(0.9, 0.8, 0.7),
            emission: v3(0.0, 0.0, 0.0),
        }
    };

    let ceiling = Sphere{
        point: v3(0.0, 200.4, 0.0),
        radius: 200.0,
        material: Material{
            color: v3(0.9, 0.8, 0.7),
            emission: v3(0.0, 0.0, 0.0),
        }
    };

    let back_wall = Sphere{
        point: v3(0.0, 0.0, -200.4),
        radius: 200.0,
        material: Material{
            color: v3(0.9, 0.8, 0.7),
            emission: v3(0.0, 0.0, 0.0),
        }
    };

    let front_wall = Sphere{
        point: v3(0.0, 0.0, 202.0),
        radius: 200.0,
        material: Material{
            color: v3(0.9, 0.8, 0.7),
            emission: v3(0.0, 0.0, 0.0),
        }
    };

    let left_sphere = Sphere{
        point: v3(-0.25, -0.24, -0.1),
        radius: 0.16,
        material: Material{
            color: v3(0.9, 0.8, 0.7),
            emission: v3(0.0, 0.0, 0.0),
        }
    };

    let right_sphere = Sphere{
        point: v3(0.25, -0.24, 0.1),
        radius: 0.16,
        material: Material{
            color: v3(0.9, 0.8, 0.7),
            emission: v3(0.0, 0.0, 0.0),
        }
    };

    let lightsource = Sphere{
        point: v3(0.0, 1.36, 0.0),
        radius: 1.0,
        material: Material{
            color: v3(0.0, 0.0, 0.0),
            emission: v3(9.0, 8.0, 6.0),
        }
    };

    scene.add(left_wall);
    scene.add(right_wall);
    scene.add(floor);
    scene.add(ceiling);
    scene.add(back_wall);
    scene.add(front_wall);
    scene.add(left_sphere);
    scene.add(right_sphere);
    scene.add(lightsource);

    let mut ppm = PPM::new(width, height);


    for x in 0..width {
        for y in 0..height {

            let mut color = v3(0.0, 0.0, 0.0);

            for _ in 0..n_samples {
                let ray = camera.ray_at((f64::from(x) + rng.gen::<f64>() - 0.5) / f64::from(width) - 0.5,
                                        (f64::from(y) + rng.gen::<f64>() - 0.5) / f64::from(height) - 0.5);

                color = color + get_light(&ray, &scene, 0, &mut rng);
            }

            color = color / f64::from(n_samples);
            color = v3(color.x.min(1.0).max(0.0).powf(1.0 / 2.2),color.y.min(1.0).max(0.0).powf(1.0 / 2.2),color.z.min(1.0).max(0.0).powf(1.0 / 2.2));
            color = color * 255.0 + 0.5;

            let pix = rgb(
                color.x as u8,
                color.y as u8,
                color.z as u8,
            );


            ppm.set(x, y, pix);
        }
    }

    if let Err(why) = ppm.save("test.ppm") {
        println!("Error: couldn't save ppm: {}", why.description());
    }
}
