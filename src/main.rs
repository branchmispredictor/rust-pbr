#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate pbr;
extern crate rand;

use pbr::*;

use std::error::Error;
use rand::Rng;

fn main() {
    let width = 1280;
    let height = 720;

    let n_samples = 20;

    let mut rng = rand::thread_rng();

    let mut camera = Camera::new(width, height);

    camera.move_to(v3(0.0, 2.0, -8.0));
    camera.look_at(v3(0.0, 0.0, 1.0));

    let mut scene = Scene::new();

    let sphere = Sphere{point: v3(0.0, 0.0, 1.0), radius: 0.5};
    let sphere2 = Sphere{point: v3(2.0, 0.0, 2.0), radius: 0.5};
    let sphere3 = Sphere{point: v3(-2.0, 0.0, 2.0), radius: 0.5};

    scene.add(sphere);
    scene.add(sphere2);
    scene.add(sphere3);

    let mut ppm = PPM::new(width, height);

    for x in 0..width {
        for y in 0..height {

            let mut color = v3(0.0, 0.0, 0.0);

            for _ in 0..n_samples {
                let ray = camera.ray_at((f64::from(x) + rng.gen::<f64>() - 0.5) / f64::from(width) - 0.5,
                                        (f64::from(y) as f64 + rng.gen::<f64>() - 0.5) / f64::from(height) - 0.5);

                if let Some(intersection) = scene.intersect(ray) {
                    color = color + intersection.object.color();
                } else {
                    let t = 0.5 * (ray.vector.y + 1.0);
                    let white = v3(1.0, 1.0, 1.0) * (1.0 - t);
                    let blue = v3(0.5, 0.7, 1.0) * t;

                    color = color + white + blue;
                }
            }

            color = color / f64::from(n_samples);
            color = color * 255.99;

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
