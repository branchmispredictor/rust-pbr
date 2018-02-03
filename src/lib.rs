// Re-exports

pub use camera::*;
pub use image::*;
pub use ray::*;
pub use shape::*;
pub use vector::*;

// Modules

mod camera;
mod image;
mod ray;
mod shape;
mod vector;

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
