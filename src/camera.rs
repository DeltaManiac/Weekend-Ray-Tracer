use crate::ray::Ray;
use crate::vec3::Vec3;
use std::f64::consts::PI;

#[derive(Copy, Clone, Default)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    //vfov is top to bottom in degrees
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Camera {
        let theta: f64 = vfov * PI / 180.0;
        let half_height: f64 = (theta / 2.0).tan();
        let half_width: f64 = aspect * half_height;
        let w = Vec3::unit_vector(look_from - look_at);
        let u = Vec3::unit_vector(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);
        Camera {
            lower_left_corner: look_from - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: look_from,
        }
    }
    pub fn get_ray(self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
