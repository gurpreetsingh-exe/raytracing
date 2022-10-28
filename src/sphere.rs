use crate::{HitRecord, Hittable, Ray};
use glm::*;

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: vec3(0.0, 0.0, 0.0),
            radius: 0.5,
        }
    }
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = dot(ray.direction, ray.direction);
        let half_b = dot(oc, ray.direction);
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sq_d = discriminant.sqrt();

        let mut root = (-half_b - sq_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sq_d) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.pos = ray.at(rec.t);
        rec.normal = (rec.pos - self.center) / self.radius;
        true
    }
}
