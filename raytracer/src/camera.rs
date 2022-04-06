use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
}

impl Default for Camera {
    fn default() -> Self {
        const ASPECT_RATIO: f64 = 16.0 / 9.0;
        const VIEWPORT_HEIGHT: f64 = 2.0;
        const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
        const FOCAL_LENGTH: f64 = 1.0;

        const ORIGIN: Point3 = Point3::new(0.0, 0.0, 0.0);
        const HORIZONTAL: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        const VERTICAL: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        // Can't be const because the overriden operator are functions that are not const
        let lower_left_corner =
            ORIGIN - HORIZONTAL / 2.0 - VERTICAL / 2.0 - Point3::new(0.0, 0.0, FOCAL_LENGTH);

        Self {
            origin: ORIGIN,
            horizontal: HORIZONTAL,
            vertical: VERTICAL,
            lower_left_corner,
        }
    }
}

impl Camera {
    pub fn shoot_ray(&self, x: f64, y: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * x + self.vertical * y,
        )
    }
}
