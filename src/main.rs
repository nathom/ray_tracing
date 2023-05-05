mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec;
use hittable::Hittable;
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use std::io::{stderr, stdout, Write};
use vec::{Color, Point3, Vec3};

// Main drawing function
// Given a fixed camera and object, calculate the color that should be displayed for a ray
fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let (hit, rec) = world.hit(r, 0.0, f64::INFINITY);
    if hit {
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

    // world

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let mut err = stderr();
    let mut out = stdout();
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {:3}", j);
        err.flush().unwrap();

        for i in 0..image_width {
            let u = (i as f64) / ((image_width - 1) as f64);
            let v = (j as f64) / ((image_height - 1) as f64);
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r, &world);
            pixel_color.write_color(&mut out).unwrap();
        }
    }
    eprintln!("\nDone.");
}
