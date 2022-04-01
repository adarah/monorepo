use crate::{ray::Ray, vec3::Color, hittable::HitRecord};

pub mod lambertian;
pub mod metal;

pub trait Material {
    fn scatter(&self, incident_ray: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Color)>;
}