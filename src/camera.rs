use glm::*;

use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(focal_length: f32, origin: Vec3, width: f32, height: f32) -> Self {
        let horizontal = vec3(width, 0.0, 0.0);
        let vertical = vec3(0.0, height, 0.0);
        Self {
            origin,
            lower_left_corner: origin
                - horizontal * 0.5
                - vertical * 0.5
                - vec3(0.0, 0.0, focal_length),
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
