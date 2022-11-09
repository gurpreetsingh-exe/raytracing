use glm::{dot, normalize, Vec3};

use crate::SEED;

pub fn rand_float() -> f32 {
    rand() as f32 / u64::MAX as f32
}

fn rand() -> u64 {
    unsafe {
        SEED ^= SEED << 13;
        SEED ^= SEED >> 17;
        SEED ^= SEED << 5;
        SEED
    }
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = glm::vec3(
            rand_float() * 2.0 - 1.0,
            rand_float() * 2.0 - 1.0,
            rand_float() * 2.0 - 1.0,
        );
        if dot(p, p) >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vec() -> Vec3 {
    normalize(random_in_unit_sphere())
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let unit_sphere = random_unit_vec();
    if dot(unit_sphere, *normal) > 0.0 {
        unit_sphere
    } else {
        -unit_sphere
    }
}
