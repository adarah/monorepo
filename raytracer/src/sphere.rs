use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::Point3,
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = *ray.origin - self.center;

        let a = ray.direction.dot(*ray.direction);
        let half_b = ray.direction.dot(oc);
        let c = oc.dot(oc) - self.radius.powi(2);
        let delta = half_b.powi(2) - a * c;

        if delta < 0.0 {
            return None;
        }

        let mut root = (-half_b - delta.sqrt()) / a;
        // Really ugly, can be better solved by a lazy generator for roots. Problem is, the generator would have no way to know of our half_b optimization
        // The generator would also require allocating memory for an iterator, and that's wasteful
        // For these reasons, I rather keep the code a little ugly, but short and efficient
        if root < t_min || root > t_max {
            root = (-half_b + delta.sqrt()) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = (point - self.center).unit();
        // Dot product of vectors in opposite directions is negative.
        let front_face = normal.dot(*ray.direction) < 0.0;

        let record = HitRecord {
            time: root,
            point,
            // For front_face to be false, the ray must have come from inside the sphere.
            // We want normal to always be opposite of the ray, so return the negation.
            normal: if front_face { normal } else { -normal },
            front_face,
        };

        return Some(record);
    }
}
