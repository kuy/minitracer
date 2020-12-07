#[macro_use]
extern crate approx;

mod hittable;
mod ray;
mod sphere;
mod vec3;

use hittable::Hittable;
use ray::Ray;
use sphere::Sphere;
use vec3::{Point3, Vec3};

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in 0..ny {
        for i in 0..nx {
            let u = f64::from(i) / f64::from(nx);
            let v = f64::from(ny - j) / f64::from(ny);

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
