use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::interval::Interval;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if front_face {outward_normal} else {Vec3::from(-1.0) * outward_normal};
    }

    pub fn t(&self) -> f32{
        self.t
    }

    pub fn p(&self) -> Vec3 {
        self.p
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }
}
pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval, rec: &mut HitRecord) -> bool;
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Sphere {
    center: Vec3,
    radious: f32
}

impl Sphere {
    pub fn new(center: Vec3, radious: f32) -> Sphere {
        Sphere { center, radious }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radious*self.radious;
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false
        }
        
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !interval.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !interval.surrounds(root) {
                return false;
            }
        }
        
        rec.t = root;
        rec.p = ray.at(rec.t);
        rec.set_face_normal(ray, (rec.p - self.center).scale(1.0/self.radious));

        return true;

    }
}



