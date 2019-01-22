mod vec3;

use vec3::Vec3;

fn main() {
    let e = Vec3::new(1.0, 2.0, 3.0);
    let mut f = Vec3::new(1.5, 2.5, 3.5);
    println!("{}", e * f);
    println!("{}", 3.0 * f);
    println!("{}", f * 3.0);
    // f -= e;
    // println!("{}", e);
    // println!("{}", f);

    let x = 200;
    let y = 100;
    print!("P3\n{} {}\n255\n", x, y);
    for j in (0..=y - 1).rev() {
        for i in 0..x {
            let col = vec3::Vec3::new(i as f64 / x as f64, j as f64 / y as f64, 0.2);
            let ir = (255.99 * col.r()) as u16;
            let ig = (255.99 * col.g()) as u16;
            let ib = (255.99 * col.b()) as u16;
            //println!("{} {} {}", ir, ig, ib)
        }
    }
}
