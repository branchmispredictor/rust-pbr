use std::fs::File;
use std::io::Write;
use std::error::Error;


struct PPM {
    width: u32,
    height: u32,
    buffer: Box<[u8]>,
}

#[derive(Debug)]
struct RGB(u8, u8, u8);

impl PPM {
    fn new(width: u32, height: u32) -> PPM {
        let size = (3 * width * height) as usize;
        let buffer: Box<[u8]> = vec![0; size].into_boxed_slice();
        PPM {width, height, buffer}
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }

    fn offset(&self, x: u32, y: u32) -> usize {
        let offset = (y * self.width + x) * 3;
        offset as usize
    }

    fn set(&mut self, x: u32, y: u32, color: RGB) {
        let offset = self.offset(x, y);
        self.buffer[offset] = color.0;
        self.buffer[offset + 1] = color.1;
        self.buffer[offset + 2] = color.2;
    }

    fn get(&self, x: u32, y: u32) -> RGB {
        let offset = self.offset(x, y);
        let r = self.buffer[offset];
        let g = self.buffer[offset + 1];
        let b = self.buffer[offset + 2];
        RGB(r, g, b)
    }

    fn save(&self, filename: &str) -> std::io::Result<()> {
        let mut file = try!(File::create(filename));
        let header = format!("P6 {} {} 255\n", self.width, self.height);
        try!(file.write(header.as_bytes()));
        try!(file.write(&self.buffer));
        Ok(())
    }
}

use std::ops::Add;
use std::ops::Mul;

#[derive(Debug)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vector3 {
    fn length(&self) -> f64 {
        let &Vector3 {x, y, z} = self;
        (x * x + y * y + z * z).sqrt()
    }

    fn normalize(&self) -> Vector3 {
        let &Vector3 {x, y, z} = self;
        let len = self.length();
        v3(x / len, y / len, z / len)
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f64> for Vector3 {
    type Output = Vector3;

    fn add(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

struct Vector4 {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Vector4 {
    fn length(&self) -> f64 {
        let &Vector4 {x, y, z, w} = self;
        (x * x + y * y + z * z + w * w).sqrt()
    }

    fn normalize(&self) -> Vector4 {
        let &Vector4 {x, y, z, w} = self;
        let len = self.length();
        v4(x / len, y / len, z / len, w / len)
    }
}

fn v3(x: f64, y: f64, z: f64) -> Vector3 {
    Vector3 {x, y, z}
}

fn v4(x: f64, y: f64, z: f64, w: f64) -> Vector4 {
    Vector4 {x, y, z, w}
}

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

            let v_pix = v * 256.0;

            let pix = RGB(
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
