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

fn generate_random_scene() -> HitableList {
    let mut rng = thread_rng();
    let mut world = HitableList::new();
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(Vec3::new(0.5, 0.5, 0.5)),
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    //Lambertian
                    world.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Lambertian::new(Vec3::new(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                        )),
                    )));
                } else if choose_mat < 0.95 {
                    // Metal
                    world.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                            ),
                            0.5 * rng.gen::<f64>(),
                        ),
                    )));
                } else {
                    world.push(Box::new(Sphere::new(center, 0.2, Dielectric::new(1.5))))
                }
            }
        }
    }
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(Vec3::new(0.4, 0.2, 0.1)),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0),
    )));
    //world.rev();
    world
}

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

fn spheres() -> HitableList {
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
    // world.push(Box::new(Sphere::new(
    //     Vec3::new(-1.0, 0.0, -1.0),
    //     -0.45,
    //     Dielectric::new(1.5),
    // )));
    world
}
fn main() {
    let x = 1920;
    let y = 1080;
    let s = 200;
    let world = generate_random_scene();
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        x as f64 / y as f64,
        aperture,
        dist_to_focus,
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
                let p = r.point_at_parameter(2.0);
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
