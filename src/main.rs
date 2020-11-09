#[macro_use]
extern crate approx;

mod vec3;

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);

    for j in 0..ny {
        for i in 0..nx {
            let r = f64::from(i) / f64::from(nx);
            let g = f64::from(ny - j) / f64::from(ny);
            let b = 0.2f64;

            let ir = (256.0 * r).round() as i32;
            let ig = (256.0 * g).round() as i32;
            let ib = (256.0 * b).round() as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
