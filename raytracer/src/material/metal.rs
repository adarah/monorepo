use crate::{hittable::HitRecord, ray::Ray, vec3::Color};

use super::Material;

pub struct Metal {
    color: Color,
}

impl Metal {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Material for Metal {
    fn scatter(&self, incident_ray: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = incident_ray.direction.reflect(&hit_rec.normal);
        if reflected.dot(hit_rec.normal) < 0.0 {
            return None;
        }
        Some((Ray::new(hit_rec.point, reflected), self.color))
    }
}
