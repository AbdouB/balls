use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::distributions::{Distribution, Uniform};

pub trait Renderable {
    fn render_pixel(&self, ray: &Ray, depth: u32, interval: Interval) -> Vec3;
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Camera {
    aspect_ratio: f32,
    image_width: u32,
    image_height: u32,
    center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
    samples_per_pixel: u32,
    max_depth: u32,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: u32, samples_per_pixel: u32, max_depth: u32) -> Camera {
        let image_height = if ((image_width as f32 / aspect_ratio) as i32) < 1 {
            1 as u32
        } else {
            (image_width as f32 / aspect_ratio) as u32
        };
    
        // camera
        let focal_length = 1.0;
        let center = Vec3::new(0.0, 0.0, 0.0);
        let viewport_height = 2.0;
        let viewport_width = (image_width as f32/image_height as f32) * viewport_height;
    
        // calculate the vectors across the horizontal and vertical axis of the viewport
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    
        // calculate the vectors across the horizontal and down the vertical axis of the viewport
        let pixel_delta_v = viewport_v / Vec3::from(image_height as f32);
        let pixel_delta_u = viewport_u / Vec3::from(image_width as f32);
    
        // calculate the location of the upper left pixel
        let viewport_upper_left = center - Vec3::new(0.0, 0.0, focal_length) - viewport_u/Vec3::from(2.0) - viewport_v/Vec3::from(2.0); 
        let pixel00_loc = viewport_upper_left + Vec3::from(0.5) * (pixel_delta_u + pixel_delta_v);
        let samples_per_pixel = if samples_per_pixel == 0 {10} else {samples_per_pixel};
        let max_depth = if max_depth == 0 {10} else {max_depth};

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn render(&self, world: &dyn Renderable) -> Result<(), &dyn std::error::Error> {
        std::println!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel = Vec3::default();
                let interval = Interval::new(0.001, f32::INFINITY);

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel = pixel + world.render_pixel(&ray, self.max_depth, interval.clone());
                }

                std::println!("{}", render_color(pixel.clone(), self.samples_per_pixel));
            }
        }
        Ok(())
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_center = self.pixel00_loc + (Vec3::from(i as f32) * self.pixel_delta_u) + (Vec3::from(j as f32) * self.pixel_delta_v);
        let pixel_sample = pixel_center + pixel_sample_square(self.pixel_delta_u, self.pixel_delta_v);
        
        let ray_direction = pixel_sample - self.center;

        Ray::new(self.center, ray_direction)
    }
}

fn render_color(pixel: Vec3, samples_per_pixel: u32) -> String {
    let scale = 1.0 / samples_per_pixel as f32;
    let intestity = Interval::new(0.000, 0.999);

    let r = (256.0 * intestity.clamp((pixel.x() * scale).sqrt())) as u32;
    let g = (256.0 * intestity.clamp((pixel.y() * scale).sqrt())) as u32;
    let b = (256.0 * intestity.clamp((pixel.z() * scale).sqrt())) as u32;
    
    format!("{} {} {}", r, g, b)
}

fn pixel_sample_square(pixel_delta_u: Vec3, pixel_delta_v: Vec3) -> Vec3 {
    let step = Uniform::new(0.0, 1.0);
    let mut rng = rand::thread_rng();
    
    let px = -0.5 + step.sample(&mut rng);
    let py = -0.5 + step.sample(&mut rng);

    (pixel_delta_u * Vec3::from(px)) + (pixel_delta_v * Vec3::from(py))
}
