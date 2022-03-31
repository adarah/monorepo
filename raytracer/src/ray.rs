use crate::{vec3::{Point3, Vec3, Color}, hittable::HitRecord};


pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub const fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, time: f64) -> Point3 {
        self.origin + time * self.direction
    }

    pub fn color(self, hit_rec: Option<HitRecord>) -> Color {
        if let Some(hit) = hit_rec {
            return 0.5 * (hit.normal + Vec3::new(1.0, 1.0, 1.0));
        }
        let dir = self.direction.unit();
        let t = 0.5 * (dir.y + 1.0);
        return (1.0 - t) * Color::WHITE + t * Color::LIGHT_BLUE;
    }
}
