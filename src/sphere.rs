use crate::hitable::{HitRecord, Hitable};
use crate::material::{Lambertian, Material};
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::{thread_rng, Rng};
use std::rc::Rc;
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
}
impl Default for Sphere {
    fn default() -> Self {
        Self::new(Vec3::default(), 0.0, Lambertian::new(Vec3::default()))
    }
}
impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = Vec3::dot(ray.direction(), ray.direction());
        let b = Vec3::dot(oc, ray.direction());
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = self.material.clone();
                return true;
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = self.material.clone();
                return true;
            }
        }
        return false;
    }
}
pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = thread_rng();
    //let mut p: Vec3 = Default::default();
    loop {
        let p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
            - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_len() >= 1.0 {
            // break;
            return p;
        }
    }
}
