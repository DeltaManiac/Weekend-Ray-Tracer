use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::{thread_rng, Rng};
use std::f64::consts::PI;

fn random_in_unit_disk() -> Vec3 {
    let mut rand = thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rand.gen::<f64>(), rand.gen::<f64>(), 0.0);
        if Vec3::dot(p, p) >= 1.0 {
            return p;
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    //vfov is top to bottom in degrees
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta: f64 = vfov * PI / 180.0;
        let half_height: f64 = (theta / 2.0).tan();
        let half_width: f64 = aspect * half_height;
        let w = Vec3::unit_vector(look_from - look_at);
        let u = Vec3::unit_vector(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);
        Camera {
            lower_left_corner: look_from
                - half_width * u * focus_dist
                - half_height * v * focus_dist
                - w * focus_dist,
            horizontal: 2.0 * half_width * u * focus_dist,
            vertical: 2.0 * half_height * v * focus_dist,
            origin: look_from,
            u: u,
            v: v,
            w: w,
            lens_radius: aperture / 2.0,
        }
    }
    pub fn get_ray(self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = rd.x() * self.u + rd.y() * self.v;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}
