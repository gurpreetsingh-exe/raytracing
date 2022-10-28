use crate::{HitRecord, Ray, Hittable};

#[derive(Default)]
pub struct World<H: Hittable> {
    objects: Vec<H>,
}

impl<H> World<H>
    where H: Hittable
{
    pub fn add(&mut self, obj: H) {
        self.objects.push(obj);
    }
}

impl<H> Hittable for World<H>
    where H: Hittable
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut hit = false;
        let mut closest = t_max;
        for obj in &self.objects {
            if obj.hit(&ray, t_min, closest, rec) {
                hit = true;
                closest = rec.t;
            }
        }
        hit
    }
}
