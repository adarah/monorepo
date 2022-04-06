use crate::{
    scene::Scene,
    vec3::{Color, Point3, Vec3},
};

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

    pub fn color(&self, scene: &Scene) -> Color {
        self.do_color(scene, 0)
    }

    fn do_color(&self, scene: &Scene, depth: usize) -> Color {
        if depth >= 50 {
            return Color::BLACK;
        }

        if let Some(hit) = scene.hit(self, 0.001, f64::MAX) {
            if let Some((scattered, attenuation)) = (*hit.material).scatter(self, &hit) {
                return attenuation * scattered.do_color(scene, depth + 1);
            }
            return Color::BLACK;
        }

        self.background_color()
    }

    fn background_color(&self) -> Color {
        let dir = self.direction.unit();
        let t = 0.5 * (dir.y + 1.0);
        (1.0 - t) * Color::WHITE + t * Color::LIGHT_BLUE
    }
}
