use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::sphere;
use crate::vec3::Vec3;
use rand::{thread_rng, Rng};
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

pub fn schlick(cosine: f64, refractive_idx: f64) -> f64 {
    let r0 = (1.0 - refractive_idx / 1.0 + refractive_idx).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * Vec3::dot(v, n) * n
}

pub fn refract(v: &Vec3, n: &Vec3, ni_over_t: f64, refracted: &mut Vec3) -> bool {
    let uv = Vec3::unit_vector(*v);
    let dt = Vec3::dot(uv, *n);
    let discriminant = 1.0 - ni_over_t.powi(2) * (1.0 - dt * dt);
    if discriminant > 0.0 {
        *refracted = ni_over_t * (uv - (*n) * dt) - (*n) * discriminant.sqrt();
        return true;
    } else {
        return false;
    }
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Rc<Lambertian> {
        Rc::new(Lambertian { albedo: albedo })
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
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
        Rc::new(Metal {
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

#[derive(Clone)]
pub struct Dielectric {
    refractive_idx: f64,
}

impl Dielectric {
    pub fn new(refractive_idx: f64) -> Rc<Dielectric> {
        Rc::new(Dielectric {
            refractive_idx: refractive_idx,
        })
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let outward_normal: Vec3; // = Default::default();
        let reflected = reflect(ray_in.direction(), hit_record.normal);
        let ni_over_t: f64; // = Default::default();
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut refracted: Vec3 = Default::default();
        //let mut reflect_prob: f64 = Default::default();
        let mut rng = thread_rng();
        let cosine = if Vec3::dot(ray_in.direction(), hit_record.normal) > 0.0 {
            outward_normal = -hit_record.normal;
            ni_over_t = self.refractive_idx;
            self.refractive_idx * Vec3::dot(ray_in.direction(), hit_record.normal)
                / ray_in.direction().len()
        } else {
            outward_normal = hit_record.normal;
            ni_over_t = 1.0 / self.refractive_idx;
            -Vec3::dot(ray_in.direction(), hit_record.normal) / ray_in.direction().len()
        };

        let reflect_prob = if refract(
            &ray_in.direction(),
            &outward_normal,
            ni_over_t,
            &mut refracted,
        ) {
            //*scattered = Ray::new(hit_record.p, refracted);
            schlick(cosine, self.refractive_idx)
        } else {
            // *scattered = Ray::new(hit_record.p, reflected);
            1.0
        };
        *scattered = if rng.gen::<f64>() >= reflect_prob {
            Ray::new(hit_record.p, refracted.clone())
        } else {
            Ray::new(hit_record.p, reflected.clone())
        };

        return true;
    }
}
