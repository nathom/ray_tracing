mod camera;

mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec;
use camera::Camera;
use hittable::Hittable;
use hittable_list::HittableList;
use rand::prelude::*;
use ray::Ray;
use sphere::Sphere;
use std::io::{stderr, stdout, Write};
use vec::{Color, Point3, Vec3};

// Main drawing function
// Given a fixed camera and object, calculate the color that should be displayed for a ray
fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        // visualize normal vectors to the surface
        return 0.5 * (rec.normal() + Color::new(1.0, 1.0, 1.0));
    }

    // otherwise, draw LERPed gradient as background
    let unit_dir = r.direction().unit_vector();
    let t = 0.5 * (unit_dir.y() + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;

    // world

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // camera

    let camera = Camera::new();

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let mut err = stderr();
    let mut out = stdout();
    // for random numbers
    let mut rng = rand::thread_rng();
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {:3}", j);
        err.flush().unwrap();

        for i in 0..image_width {
            let mut pixel_color = Color::zero();
            for _ in 0..samples_per_pixel {
                // take random samples of the (i,j) pixel in the range ([i, i+1), [j, j+1))
                let r1: f64 = rng.gen();
                let r2: f64 = rng.gen();

                // get ray to render
                let u = (i as f64 + r1) / (image_width as f64 - 1.0);
                let v = (j as f64 + r2) / (image_height as f64 - 1.0);
                let r = camera.get_ray(u, v);

                // accumulate color, to be descaled by writer
                pixel_color += ray_color(&r, &world);
            }
            // descale, write pixel
            pixel_color
                .write_color_descaled(&mut out, samples_per_pixel)
                .unwrap();
        }
    }
    eprintln!("\nDone.");
}
