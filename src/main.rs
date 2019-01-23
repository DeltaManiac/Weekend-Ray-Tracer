extern crate rand;

mod camera;
mod hitable;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hitable::{HitRecord, Hitable, HitableList};
use material::{Dielectric, Lambertian, Metal};
use rand::{thread_rng, Rng};
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn color(ray: &Ray, world: &HitableList, depth: i16) -> Vec3 {
    let mut rec = HitRecord::new();
    if world.hit(ray, 0.001, std::f64::MAX, &mut rec) {
        let mut scattered: Ray = Default::default();
        let mut attenuation: Vec3 = Default::default();
        if depth < 50
            && rec
                .material
                .scatter(ray, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * color(&scattered, world, depth + 1);
        } else {
            return Vec3::default();
        }
    } else {
        let unit_dir = Vec3::unit_vector(ray.direction());
        let t = 0.5 * (unit_dir.y() + 1.0);
        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
}

fn main() {
    let x = 800;
    let y = 400;
    let s = 500;
    let mut world = HitableList::new();
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Lambertian::new(Vec3::new(0.1, 0.2, 0.5)),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Lambertian::new(Vec3::new(0.8, 0.8, 0.0)),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Dielectric::new(1.5),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        Dielectric::new(1.5),
    )));
    let camera = Camera::new(
        Vec3::new(-2.0, 2.0, -1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        x as f64 / y as f64,
    );
    let mut rng = thread_rng();
    print!("P3\n{} {}\n255\n", x, y);
    for j in (0..=y - 1).rev() {
        for i in 0..x {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _k in 0..s {
                let u = ((i as f64) + rng.gen::<f64>()) / x as f64;
                let v = ((j as f64) + rng.gen::<f64>()) / y as f64;
                let r = camera.get_ray(u, v);
                //let p = r.point_at_parameter(2.0);
                col += color(&r, &world, 0);
            }
            col /= s as f64;
            col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());
            let ir = (255.99 * col.r()) as u16;
            let ig = (255.99 * col.g()) as u16;
            let ib = (255.99 * col.b()) as u16;
            println!("{} {} {}", ir, ig, ib)
        }
    }
}
