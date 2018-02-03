// Re-exports

pub use image::*;
pub use vector::*;

// Modules

mod image;
mod vector;

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
