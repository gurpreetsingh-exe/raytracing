pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod world;

use std::fs::File;
use std::io::{BufWriter, Write};

use glm::{Vec3, vec3, normalize};
use ray::Ray;
use hittable::{Hittable, HitRecord};
use sphere::Sphere;
use world::World;

const WIDTH: i32 = 400;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const HEIGHT: i32 = (WIDTH as f32 / ASPECT_RATIO) as i32;

fn write_color(buf: &mut Vec<u8>, color: Vec3) {
    buf.append(
        &mut format!(
            "{} {} {}\n",
            (color.x * 255.999) as i32,
            (color.y * 255.999) as i32,
            (color.z * 255.999) as i32
        )
        .into_bytes(),
    )
}

fn ray_color<H: Hittable>(ray: &Ray, world: &World<H>) -> Vec3 {
    let mut hit_rec = HitRecord::default();
    if world.hit(ray, 0.0, f32::INFINITY, &mut hit_rec) {
        (hit_rec.normal + vec3(1.0, 1.0, 1.0)) * 0.5
    } else {
        let unit_direction = normalize(ray.direction);
        let t = 0.5 * (unit_direction.y + 1.0);
        vec3(1.0, 1.0, 1.0) * (1.0 - t) + vec3(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    let mut data: Vec<u8> = format!("P3\n{} {}\n255\n", WIDTH, HEIGHT).into_bytes();

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = vec3(0.0, 0.0, 0.0);
    let horizontal = vec3(viewport_width, 0.0, 0.0);
    let vertical = vec3(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - vec3(0.0, 0.0, focal_length);

    let mut world = World::<Sphere>::default();
    world.add(Sphere::new(vec3(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(vec3(0.0, -100.5, -1.0), 100.0));

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let u = i as f32 / (WIDTH - 1) as f32;
            let v = (HEIGHT - j - 1) as f32 / (HEIGHT - 1) as f32;
            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);
            let pixel_color = ray_color(&ray, &world);
            write_color(&mut data, pixel_color);
        }
    }

    let f = File::create("image.ppm").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    let buf = data.as_slice();
    f.write_all(buf)
        .expect("Unable to write data");
}
