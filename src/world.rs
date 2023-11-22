use crate::geometry::{
    Hittable,
    HitRecord
};
use crate::ray::Ray;
use crate::vec3::{
    Vec3,
    self
};
use crate::interval::Interval;
use crate::camera::Renderable;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct World<T: Hittable> {
    objects: std::vec::Vec<T>,
}

impl<T> World<T> where T: Hittable {
    pub fn new(objects: std::vec::Vec<T>) -> World<T> {
        World {
            objects
        }
    }
}

impl<T> Renderable for World<T> where T: Hittable {
    fn render_pixel(&self, ray: &Ray, depth: u32, interval: Interval) -> Vec3 {
        if depth == 0 {
            return Vec3::default();
        }

        let mut rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = interval.max();

        for object in self.objects.iter() {
            if object.hit(ray, Interval::new(interval.min(), closest_so_far), &mut rec) {
                hit_anything = true;
                closest_so_far = rec.t();
            }
        }

        let color: Vec3 = if hit_anything{
            let direction = rec.normal() + vec3::random_unit_vector();
            Vec3::from(0.1) * self.render_pixel(&Ray::new(rec.p(), direction), depth-1, interval.clone())
        } else {
            let unit_direction = Vec3::from(ray.direction());
            let a = 0.5*(unit_direction.y() + 1.0);
    
            Vec3::from(1.0-a) * Vec3::from(1.0) + Vec3::from(a)*Vec3::new(0.5, 0.7, 1.0)
        };

        color
    }
}

