#[macro_use]
extern crate approx;

mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use hittable::Hittable;
use ray::Ray;
use sphere::Sphere;
use vec3::{Point3, Vec3};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (f64::from(image_width) / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin.clone()
        - (horizontal.clone() * 0.5)
        - (vertical.clone() * 0.5)
        - Vec3::new(0.0, 0.0, focal_length);

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let u = f64::from(i) / f64::from(image_width - 1);
            let v = f64::from(image_height - j) / f64::from(image_height - 1);

            let r = Ray::new(
                origin.clone(),
                lower_left_corner.clone() + u * horizontal.clone() + v * vertical.clone(),
            );
            let col = ray_color(r);

            let ir = (255.0 * col.r()).round() as i32;
            let ig = (255.0 * col.g()).round() as i32;
            let ib = (255.0 * col.b()).round() as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn ray_color(r: Ray) -> Vec3 {
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    if let Some(rec) = sphere.hit(&r, -10.0, 10.0) {
        let n = (r.at(rec.t) - Vec3::new(0.0, 0.0, -1.0)).to_unit();
        return Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) / 2.0;
    }
    let unit_dir = r.direction().to_unit();
    let t = (unit_dir.y() + 1.0) / 2.0;
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
