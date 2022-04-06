use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

use super::Material;

pub struct Lambertian {
    color: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _incident_ray: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit_rec.normal + Vec3::rand_in_unit_sphere().unit();
        if scatter_direction.near_zero() {
            scatter_direction = hit_rec.normal;
        }
        Some((Ray::new(hit_rec.point, scatter_direction), self.color))
    }
}
