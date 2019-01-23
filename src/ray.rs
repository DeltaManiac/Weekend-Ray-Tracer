use crate::Vec3;

#[derive(Copy, Clone, Default, Debug)]
pub struct Ray {
    org: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(org: Vec3, dir: Vec3) -> Ray {
        Ray { org: org, dir: dir }
    }

    pub fn origin(self) -> Vec3 {
        self.org
    }

    pub fn direction(self) -> Vec3 {
        self.dir
    }

    pub fn point_at_parameter(self, t: f64) -> Vec3 {
        self.org + t * self.dir
    }
}
