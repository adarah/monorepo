mod hittable;
mod ray;
mod scene;
mod sphere;
mod vec3;

use anyhow::Result;
use hittable::HitRecord;
use vec3::Color;

use crate::{
    hittable::Hittable,
    ray::Ray,
    scene::Scene,
    sphere::Sphere,
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

    let scene = Scene::new(vec![
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
    ]);

    print!("P3\n{IMG_WIDTH} {IMG_HEIGHT}\n255\n");
    for j in (0..IMG_HEIGHT).rev() {
        eprintln!("Lines remaining: {j}");
        for i in 0..IMG_WIDTH {
            let x_norm = (i as f64) / (IMG_WIDTH - 1) as f64;
            let y_norm = (j as f64) / (IMG_HEIGHT - 1) as f64;

            let direction = lower_left_corner + x_norm * horizontal + y_norm * vertical - origin;
            let r = Ray::new(&origin, &direction);
            let hit_rec = scene.hit(&r, 0.0, f64::MAX);
            println!("{}", r.color(hit_rec));
        }
    }

    Ok(())
}

impl<'a> Ray<'a> {
    pub fn color(self, hit_rec: Option<HitRecord>) -> Color {
        if let Some(hit) = hit_rec {
            return 0.5 * (hit.normal + Vec3::new(1.0, 1.0, 1.0));
        } 
        let dir = self.direction.unit();
        let t = 0.5 * (dir.y + 1.0);
        return (1.0 - t) * WHITE + t * BLUE;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn make_bazel_happy() {
        assert_eq!(1 + 1, 2);
    }
}
