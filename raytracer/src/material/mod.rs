use crate::{hittable::HitRecord, ray::Ray, vec3::Color};

pub mod lambertian;
pub mod metal;

pub trait Material {
    fn scatter(&self, incident_ray: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Color)>;
}
