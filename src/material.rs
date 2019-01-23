use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::sphere;
use crate::vec3::Vec3;
use std::rc::Rc;

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * Vec3::dot(v, n) * n
}

#[derive(Clone, Debug)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> std::rc::Rc<Lambertian> {
        std::rc::Rc::new(Lambertian { albedo: albedo })
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = hit_record.p + hit_record.normal + sphere::random_in_unit_sphere();
        *scattered = Ray::new(hit_record.p, target - hit_record.p);
        *attenuation = self.albedo;
        return true;
    }
}
#[derive(Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}
impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Rc<Metal> {
        std::rc::Rc::new(Metal {
            albedo: albedo,
            fuzz: fuzz,
        })
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(Vec3::unit_vector(ray_in.direction()), hit_record.normal);
        *scattered = Ray::new(
            hit_record.p,
            reflected + self.fuzz * sphere::random_in_unit_sphere(),
        );
        *attenuation = self.albedo;
        return Vec3::dot(scattered.direction(), hit_record.normal) > 0.0;
    }
}
