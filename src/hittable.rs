use glm::{Vec3, vec3};
use crate::ray::Ray;

#[derive(Debug)]
pub struct HitRecord {
    pub pos: Vec3,
    pub normal: Vec3,
    pub t: f32,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self { pos: vec3(0.0, 0.0, 0.0), normal: vec3(0.0, 0.0, 0.0), t: 0.0 }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
