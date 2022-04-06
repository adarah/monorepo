use std::rc::Rc;

use crate::{
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub time: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
