mod camera;
mod hittable;
mod ray;
mod scene;
mod sphere;
mod vec3;

use anyhow::Result;

use crate::{camera::Camera, scene::Scene, sphere::Sphere, vec3::Point3};

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMG_WIDTH: usize = 400;
const IMG_HEIGHT: usize = ((IMG_WIDTH as f64) / ASPECT_RATIO) as usize;

fn main() -> Result<()> {
    let scene = Scene::new(vec![
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
    ]);

    let camera: Camera = Default::default();

    print!("P3\n{IMG_WIDTH} {IMG_HEIGHT}\n255\n");
    for j in (0..IMG_HEIGHT).rev() {
        eprintln!("Lines remaining: {j}");
        for i in 0..IMG_WIDTH {
            let x_norm = (i as f64 + rand::random::<f64>()) / (IMG_WIDTH - 1) as f64;
            let y_norm = (j as f64 + rand::random::<f64>()) / (IMG_HEIGHT - 1) as f64;

            let r = camera.shoot_ray(x_norm, y_norm);
            let hit_rec = scene.hit(&r, 0.0, f64::MAX);
            println!("{}", r.color(hit_rec));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn make_bazel_happy() {
        assert_eq!(1 + 1, 2);
    }
}
