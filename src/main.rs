#[macro_use]
extern crate approx;

mod vec3;

use vec3::Vec3;

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);

    for j in 0..ny {
        for i in 0..nx {
            let col = Vec3::new(
                f64::from(i) / f64::from(nx),
                f64::from(ny - j) / f64::from(ny),
                0.2,
            );

            let ir = (255.0 * col.r()).round() as i32;
            let ig = (255.0 * col.g()).round() as i32;
            let ib = (255.0 * col.b()).round() as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
