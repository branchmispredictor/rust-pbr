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

// Generate cornell "sphere" scene, align camera
pub fn cornell_sphere_scene<'a>(width: u32, height: u32) -> (Camera, Scene<'a>) {
    let mut camera = Camera::new(width, height);
    camera.move_to(v3(0.0, 0.1, 2.0));
    camera.look_at(v3(0.0, 0.1, 0.0));

    let mut scene = Scene::new();

    let left_wall = Sphere{
        point: v3(-200.6, 0.0, 0.0),
        radius: 200.0,
        material: Material{
            color: v3(0.75, 0.25, 0.25),
            emission: v3(0.0, 0.0, 0.0),
        }
    };

    let right_wall = Sphere{
        point: v3(200.6, 0.0, 0.0),
        radius: 200.0,
        material: Material{
            color: v3(0.25, 0.25, 0.75),
            emission: v3(0.0, 0.0, 0.0),
        }
    };

    let floor = Sphere{
        point: v3(0.0, -200.4, 0.0),
        radius: 200.0,
        material: Material{
            color: v3(0.9, 0.8, 0.7),
            emission: v3(0.0, 0.0, 0.0),
        }
    };

    let ceiling = Sphere{
        point: v3(0.0, 200.4, 0.0),
        radius: 200.0,
        material: Material{
            color: v3(0.9, 0.8, 0.7),
            emission: v3(0.0, 0.0, 0.0),
        }
    };

    let back_wall = Sphere{
        point: v3(0.0, 0.0, -200.4),
        radius: 200.0,
        material: Material{
            color: v3(0.9, 0.8, 0.7),
            emission: v3(0.0, 0.0, 0.0),
        }
    };

    let front_wall = Sphere{
        point: v3(0.0, 0.0, 202.0),
        radius: 200.0,
        material: Material{
            color: v3(0.9, 0.8, 0.7),
            emission: v3(0.0, 0.0, 0.0),
        }
    };

    let left_sphere = Sphere{
        point: v3(-0.25, -0.24, -0.1),
        radius: 0.16,
        material: Material{
            color: v3(0.9, 0.8, 0.7),
            emission: v3(0.0, 0.0, 0.0),
        }
    };

    let right_sphere = Sphere{
        point: v3(0.25, -0.24, 0.1),
        radius: 0.16,
        material: Material{
            color: v3(0.9, 0.8, 0.7),
            emission: v3(0.0, 0.0, 0.0),
        }
    };

    let lightsource = Sphere{
        point: v3(0.0, 1.36, 0.0),
        radius: 1.0,
        material: Material{
            color: v3(0.0, 0.0, 0.0),
            emission: v3(9.0, 8.0, 6.0),
        }
    };

    scene.add(left_wall);
    scene.add(right_wall);
    scene.add(floor);
    scene.add(ceiling);
    scene.add(back_wall);
    scene.add(front_wall);
    scene.add(left_sphere);
    scene.add(right_sphere);
    scene.add(lightsource);

    (camera, scene)
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
