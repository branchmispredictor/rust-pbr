#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

// Re-exports

pub use camera::*;
pub use image::*;
pub use material::*;
pub use object::*;
pub use ray::*;
pub use scene::*;
pub use vector::*;

// Modules

mod camera;
mod image;
mod material;
mod object;
mod ray;
mod scene;
mod vector;

// Functions

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
