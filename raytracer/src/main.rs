mod ray;
mod vec3;

use anyhow::Result;
use vec3::Color;

use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMG_WIDTH: usize = 400;
const IMG_HEIGHT: usize = ((IMG_WIDTH as f64) / ASPECT_RATIO) as usize;

// Camera
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

// Colors
const RED: Color = Color::new(1.0, 0.0, 0.0);
const GREEN: Color = Color::new(0.0, 1.0, 0.0);
const BLUE: Color = Color::new(0.5, 0.7, 1.0);
const WHITE: Color = Color::new(1.0, 1.0, 1.0);

fn main() -> Result<()> {
    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let depth = Vec3::new(0.0, 0.0, FOCAL_LENGTH);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - depth;

    print!("P3\n{IMG_WIDTH} {IMG_HEIGHT}\n255\n");
    for j in (0..IMG_HEIGHT).rev() {
        eprintln!("Lines remaining: {j}");
        for i in 0..IMG_WIDTH {
            let x_norm = (i as f64) / (IMG_WIDTH - 1) as f64;
            let y_norm = (j as f64) / (IMG_HEIGHT - 1) as f64;

            let direction = lower_left_corner + x_norm * horizontal + y_norm * vertical - origin;
            let r = Ray::new(&origin, &direction);
            println!("{}", r.color());
        }
    }

    Ok(())
}

impl<'a> Ray<'a> {
    pub fn color(self) -> Color {
        let dir = self.direction.unit();
        let sphere_center = Point3::new(0.0, 0.0, -1.0);
        if let Some(p) = self.hit_sphere(sphere_center, 0.5) {
            let n = (p - sphere_center).unit();
            return 0.5 * (n + Vec3::new(1.0, 1.0, 1.0));
        }
        let t = 0.5 * (dir.y + 1.0);
        return (1.0 - t) * WHITE + t * BLUE;
    }

    fn hit_sphere(self, center: Point3, radius: f64) -> Option<Point3> {
        let oc = *self.origin - center;

        let a = self.direction.dot(*self.direction);
        let b = (2.0 * *self.direction).dot(oc);
        let c = oc.dot(oc) - radius.powi(2); 
        let delta = b.powi(2) - 4.0 * a * c;

        if delta < 0.0 {
            return None
        }
        let root = (-b - delta.sqrt()) / (2.0 * a);

        return Some(self.at(root))
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn make_bazel_happy() {
        assert_eq!(1 + 1, 2);
    }
}
