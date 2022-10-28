pub mod camera;
pub mod hittable;
pub mod ray;
pub mod sphere;
pub mod utils;
pub mod world;

use std::fs::File;
use std::io::{BufWriter, Write};

use camera::Camera;
use glm::{clamp, normalize, vec3, Vec3};
use hittable::{HitRecord, Hittable};
use ray::Ray;
use sphere::Sphere;
use utils::rand_float;
use world::World;

use std::time::{SystemTime, UNIX_EPOCH};

const WIDTH: i32 = 640;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const HEIGHT: i32 = (WIDTH as f32 / ASPECT_RATIO) as i32;

static mut SEED: u64 = 0;

fn write_color(buf: &mut Vec<u8>, color: Vec3, samples: u8) {
    let scale = 1.0 / samples as f32;
    let p_color = clamp(
        color * scale,
        vec3(0.0, 0.0, 0.0),
        vec3(0.999, 0.999, 0.999),
    );

    buf.append(
        &mut format!(
            "{} {} {}\n",
            (p_color.x * 256.0) as i32,
            (p_color.y * 256.0) as i32,
            (p_color.z * 256.0) as i32
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

    let camera = Camera::new(focal_length, origin, viewport_width, viewport_height);

    let mut world = World::<Sphere>::default();
    world.add(Sphere::new(vec3(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(vec3(0.0, -100.5, -1.0), 100.0));

    unsafe {
        SEED = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64() as u64
    };
    let samples = 20;

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let mut pixel_color = vec3(0.0, 0.0, 0.0);
            for _ in 0..samples {
                let u = (i as f32 + rand_float()) / (WIDTH - 1) as f32;
                let v = ((HEIGHT - j - 1) as f32 + rand_float()) / (HEIGHT - 1) as f32;
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &world);
            }
            write_color(&mut data, pixel_color, samples);
        }
    }

    let f = File::create("image.ppm").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    let buf = data.as_slice();
    f.write_all(buf).expect("Unable to write data");
}
