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
