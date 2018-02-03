extern crate pbr;

use pbr::*;

use std::error::Error;

fn main() {
    let width = 1280;
    let height = 720;

    let mut ppm = PPM::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let v = v3(
                (x as f64) / (width as f64),
                (y as f64) / (height as f64),
                0.2,
            );

            let v_pix = &v * 256.0;

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
