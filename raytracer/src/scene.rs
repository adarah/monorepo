use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct Scene {
    objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        Scene { objects }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut t_max = t_max;
        for obj in self.objects.iter() {
            let curr_hit = obj.hit(ray, t_min, t_max);
            match (&curr_hit, &closest_hit) {
                (Some(x), None) => {
                    t_max = x.time;
                    closest_hit = curr_hit;
                }
                (Some(x), Some(y)) if x.time < y.time => {
                    t_max = x.time;
                    closest_hit = curr_hit;
                }
                _ => (),
            }
        }
        return closest_hit;
    }
}
