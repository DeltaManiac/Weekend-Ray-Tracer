use crate::ray::Ray;
use crate::vec3::Vec3;
#[derive(Default, Copy, Clone, Debug)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HitableList {
    hit_list: Vec<Box<Hitable>>,
}

impl HitableList {
    pub fn new() -> HitableList {
        HitableList {
            hit_list: Vec::new(),
        }
    }

    pub fn push(&mut self, hitable: Box<Hitable>) {
        self.hit_list.push(hitable)
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = Default::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for i in &self.hit_list {
            if i.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
                rec.t = temp_rec.t;
            }
        }
        hit_anything
    }
}
