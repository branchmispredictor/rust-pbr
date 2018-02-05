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

        // Generate new ray dir with random numbers, offset the point slightly to prevent bouncing directly back
        let ray_dir = u*rand1.cos()*rand2s + v*rand1.sin()*rand2s + w*(1.0 - rand2).sqrt();
        let new_ray = Ray::new(intersection.point + intersection.normal * DIST_EPSILON, ray_dir);


        // Next color is cosine-weighted importance sampling for difffuse
        // Times color of the current hit
        //let weight = ray_dir.dot(&intersection.normal);
        return color + &(material.color/* * weight*/) * &get_light(&new_ray, scene, depth+1, rng);
    }

    let t = 0.5 * (ray.vector.y + 1.0);
    let white = v3(1.0, 1.0, 1.0) * (1.0 - t);
    let blue = v3(0.5, 0.7, 1.0) * t;

    white + blue
}

// Takes a random probability from 0 -> 1 and plugs it into
// the inverse cdf of a triangle filter to get x value of the
// filter (-1 to 1)
fn triangle_filter_icdf(rand: f64) -> f64 {
    let n = 2.0 * rand;
    if n < 1.0 {
        n.sqrt() - 1.0
    } else {
        1.0 - (2.0 - n).sqrt()
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let width = 640;
    let height = 480;

    let n_samples = 128/4;

    let (camera, scene) = cornell_sphere_scene(width, height);

    let mut ppm = PPM::new(width, height);

    for x in 0..width {
        for y in 0..height {

            let mut color = v3(0.0, 0.0, 0.0);

            for sy in 0..2 { // 2x2 subpixel rows for antialiasing
                for sx in 0..2 { // 2x2 subpixel cols for antialiasing
                    for _ in 0..n_samples {
                        // Generate points along a tent/triangle filter
                        // https://computergraphics.stackexchange.com/questions/3868/why-use-a-tent-filter-in-path-tracing
                        let dx = triangle_filter_icdf(rng.gen::<f64>());
                        let dy = triangle_filter_icdf(rng.gen::<f64>());

                        let antialias_off_x = (f64::from(sx) + 0.5 + dx) / 2.0;
                        let antialias_off_y = (f64::from(sy) + 0.5 + dy) / 2.0;

                        let u = (antialias_off_x + f64::from(x)) / f64::from(width) - 0.5;
                        let v = (antialias_off_y + f64::from(y)) / f64::from(height) - 0.5;

                        let ray = camera.ray_at(u, v);
                        color = color + get_light(&ray, &scene, 0, &mut rng) / 4.0;
                    }
                }
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
