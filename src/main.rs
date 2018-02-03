extern crate pbr;

use pbr::*;

use std::error::Error;

fn main() {
    let width = 1280;
    let height = 720;

    let mut camera = Camera::new(width, height, 45.0);
    /*let camera = Camera{
        point: v3(0.0, 0.0, -10.0),
        fieldOfView: 45.0,
        vector: v3(0.0, 0.0, 1.0),
    };*/
    &camera.move_to(v3(0.0, 5.0, -10.0));
    &camera.look_at(v3(0.0, 0.0, 1.0));

    let eye_vector = camera.vector;
    let vpRight = &eye_vector.cross(&V3_UP).normalize();
    let vpUp = &vpRight.cross(&eye_vector).normalize();

    let fovRadians = camera.fovRadians;
    let heightWidthRatio = height as f64 / width as f64;
    let halfWidth = fovRadians.tan();
    let halfHeight = heightWidthRatio * halfWidth;
    let camerawidth = halfWidth * 2.0;
    let cameraheight = halfHeight * 2.0;
    let pixelWidth = camerawidth / (width - 1) as f64;
    let pixelHeight = cameraheight / (height - 1) as f64;

    let mut ppm = PPM::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let xcomp = camera.v_right * (x as f64 * pixelWidth - halfWidth);
            let ycomp = camera.v_up * (y as f64 * pixelHeight - halfHeight);

            let color = Ray::new(camera.point, eye_vector + xcomp + ycomp).color();

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
