use crate::geometry::Sphere;
use crate::vec3::Vec3;
use crate::world::World;
use crate::camera::Camera;

mod ray;
mod geometry;
mod vec3;
mod world;
mod interval;
mod camera;

fn main() {
    // image  that has atleast a  height of 1 pixel
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400; 
    let samples_per_pixel = 100;
    let max_depth = 50;

    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    let world = World::new(vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)
    ]);

    match camera.render(&world) {
        Ok(_) => {}
        Err(err) => {
            std::println!("Error rendering: {}", err);
        }
    }
}
