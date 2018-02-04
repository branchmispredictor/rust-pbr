use std::fs::File;
use std::io::Write;

#[derive(Debug, Copy, Clone)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn rgb(r: u8, g: u8, b: u8) -> RGB {
    RGB{r, g, b}
}

pub struct PPM {
    width: u32,
    height: u32,
    buffer: Box<[u8]>,
}

impl PPM {
    pub fn new(width: u32, height: u32) -> PPM {
        let size = (3 * width * height) as usize;
        let buffer: Box<[u8]> = vec![0; size].into_boxed_slice();
        PPM {width, height, buffer}
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn offset(&self, x: u32, y: u32) -> usize {
        let real_y = self.height - y - 1;
        let offset = (real_y * self.width + x) * 3;
        offset as usize
    }

    pub fn set(&mut self, x: u32, y: u32, color: RGB) {
        let offset = self.offset(x, y);
        self.buffer[offset] = color.r;
        self.buffer[offset + 1] = color.g;
        self.buffer[offset + 2] = color.b;
    }

    pub fn get(&self, x: u32, y: u32) -> RGB {
        let offset = self.offset(x, y);
        let r = self.buffer[offset];
        let g = self.buffer[offset + 1];
        let b = self.buffer[offset + 2];
        rgb(r, g, b)
    }

    pub fn save(&self, filename: &str) -> ::std::io::Result<()> {
        let mut file = try!(File::create(filename));
        let header = format!("P6 {} {} 255\n", self.width, self.height);
        try!(file.write_all(header.as_bytes()));
        try!(file.write_all(&self.buffer));
        Ok(())
    }
}
