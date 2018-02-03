extern crate pbr;

use pbr::*;

use std::error::Error;

fn main() {
    let width = 1280;
    let height = 720;

    let mut camera = Camera::new(width, height);

    &camera.move_to(v3(0.0, 5.0, -10.0));
    &camera.look_at(v3(0.0, 0.0, 1.0));


    let mut ppm = PPM::new(width, height);

    for x in 0..width {
        for y in 0..height {

            let ray = camera.ray_at(x as f64 / width as f64 - 0.5,
                                    y as f64 / height as f64 - 0.5);

            let color = ray.color();

            let v_pix = color * 255.9;

            let pix = rgb(
                v_pix.x as u8,
                v_pix.y as u8,
                v_pix.z as u8,
            );


            ppm.set(x, y, pix);
        }
    }

    match ppm.save("test.ppm") {
        Err(why) => println!("Error: couldn't save ppm: {}", why.description()),
        Ok(_) => ()
    };
}
