use std::default::Default;
use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone, Default)]
struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    fn new(e1: f64, e2: f64, e3: f64) -> Vec3 {
        Vec3 { e: [e1, e2, e3] }
    }
    fn x(self) -> f64 {
        self.e[0]
    }
    fn y(self) -> f64 {
        self.e[1]
    }
    fn z(self) -> f64 {
        self.e[2]
    }

    fn r(self) -> f64 {
        self.e[0]
    }
    fn g(self) -> f64 {
        self.e[1]
    }
    fn b(self) -> f64 {
        self.e[2]
    }

    fn make_unit_vector(&mut self) -> &Vec3 {
        let length = 1.0 / (self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)).sqrt();
        self.e[0] *= length;
        self.e[1] *= length;
        self.e[2] *= length;
        self
    }

    fn add(&mut self, b: Vec3) -> &Vec3 {
        self.e[0] += b.e[0];
        self.e[1] += b.e[1];
        self.e[2] += b.e[2];
        self
    }
    fn sub(&mut self, b: Vec3) -> &Vec3 {
        self.e[0] -= b.e[0];
        self.e[1] -= b.e[1];
        self.e[2] -= b.e[2];
        self
    }
    fn mul(&mut self, b: Vec3) -> &Vec3 {
        self.e[0] *= b.e[0];
        self.e[1] *= b.e[1];
        self.e[2] *= b.e[2];
        self
    }

    fn mul_scalar(&mut self, b: f64) -> &Vec3 {
        self.e[0] *= b;
        self.e[1] *= b;
        self.e[2] *= b;
        self
    }
    fn div(&mut self, b: Vec3) -> &Vec3 {
        self.e[0] /= b.e[0];
        self.e[1] /= b.e[1];
        self.e[2] /= b.e[2];
        self
    }

    fn div_scalar(&mut self, b: f64) -> &Vec3 {
        self.e[0] /= b;
        self.e[1] /= b;
        self.e[2] /= b;
        self
    }

    fn dot(self, b: Vec3) -> f64 {
        self.e[0] * b.e[0] + self.e[1] * b.e[1] + self.e[2] * b.e[2]
    }

    fn cross(self, b: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * b.e[2] - self.e[2] * b.e[1],
                -(self.e[0] * b.e[2] - self.e[2] * b.e[0]),
                self.e[0] * b.e[1] - self.e[1] * b.e[0],
            ],
        }
    }

    fn len(self) -> f64 {
        (self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)).sqrt()
    }
    fn squared_len(self) -> f64 {
        self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

fn main() {
    let x = 200;
    let y = 100;
    print!("P3\n{} {}\n255\n", x, y);
    for j in (0..=y - 1).rev() {
        for i in 0..x {
            let col = Vec3::new(i as f64 / x as f64, j as f64 / y as f64, 0.2);
            let ir = (255.99 * col.r()) as u16;
            let ig = (255.99 * col.g()) as u16;
            let ib = (255.99 * col.b()) as u16;
            println!("{} {} {}", ir, ig, ib)
        }
    }
}
