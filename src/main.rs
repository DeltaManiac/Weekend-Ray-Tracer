mod ray;
mod vec3;
use ray::Ray;
use vec3::Vec3;

fn color(ray: Ray) -> Vec3 {
    let unit_dir = Vec3::unit_vector(ray.direction());
    let t: f64 = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let x = 200;
    let y = 100;
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    print!("P3\n{} {}\n255\n", x, y);
    for j in (0..=y - 1).rev() {
        for i in 0..x {
            let _u = i as f64 / x as f64;
            let _v = j as f64 / y as f64;
            let _r = Ray::new(origin, lower_left_corner + _u * horizontal + _v * vertical);
            let col = color(_r);
            let ir = (255.99 * col.r()) as u16;
            let ig = (255.99 * col.g()) as u16;
            let ib = (255.99 * col.b()) as u16;
            println!("{} {} {}", ir, ig, ib)
        }
    }
}
