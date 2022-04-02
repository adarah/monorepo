mod camera;
mod hittable;
mod material;
mod ray;
mod scene;
mod sphere;
mod vec3;

use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use rand::{prelude::SmallRng, Rng, SeedableRng};

use crate::{
    camera::Camera,
    material::{lambertian::Lambertian, metal::Metal, Material},
    scene::Scene,
    sphere::Sphere,
    vec3::{Color, Point3},
};

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMG_WIDTH: usize = 2560;
const IMG_HEIGHT: usize = ((IMG_WIDTH as f64) / ASPECT_RATIO) as usize;
const AA_SAMPLES: usize = 8;

thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::from_entropy())
}

fn main() -> Result<()> {
    let red_lamb = Lambertian::new(Color::RED);
    let green_lamb = Lambertian::new(Color::GREEN);
    let silver_metal: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let gold_metal: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));
    let scene = Scene::new(vec![
        Box::new(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Rc::new(red_lamb),
        )),
        Box::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            Rc::clone(&silver_metal),
        )),
        Box::new(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            Rc::clone(&gold_metal),
        )),
        Box::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Rc::new(green_lamb),
        )),
    ]);

    let camera: Camera = Default::default();

    print!("P3\n{IMG_WIDTH} {IMG_HEIGHT}\n255\n");
    for j in (0..IMG_HEIGHT).rev() {
        eprintln!("Lines remaining: {j}");
        for i in 0..IMG_WIDTH {
            let color = get_pixel_color(&camera, &scene, i, j, AA_SAMPLES);
            println!("{color}");
        }
    }

    Ok(())
}

fn get_pixel_color(camera: &Camera, scene: &Scene, pixel_x: usize, pixel_y: usize, num_samples: usize) -> Color {
    let mut c = Color::BLACK;
    for _ in 0..num_samples {
        let rand_0 = RNG.with(|rng| rng.borrow_mut().gen_range(0.0..1.0));
        let rand_1 = RNG.with(|rng| rng.borrow_mut().gen_range(0.0..1.0));
        let x_norm = (pixel_x as f64 + rand_0) / (IMG_WIDTH - 1) as f64;
        let y_norm = (pixel_y as f64 + rand_1) / (IMG_HEIGHT - 1) as f64;

        let r = camera.shoot_ray(x_norm, y_norm);
        c += r.color(&scene);
    }
    return c / num_samples as f64;
}

#[cfg(test)]
mod tests {
    #[test]
    fn make_bazel_happy() {
        assert_eq!(1 + 1, 2);
    }
}
