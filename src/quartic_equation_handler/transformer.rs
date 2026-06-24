use crate::cubic_equation_handler::transformer;
use crate::quadratic_equation_handler::general;

pub enum Equations {
    Cubic(transformer::Cubiceqn),
    Biquadratic(general::BiquadraticDegree4),
}
pub struct Quartic {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
}

pub struct NormalizedQuartic {
    // A = 1
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
}
pub trait Normalize {
    fn normalize(&self) -> NormalizedQuartic;
}

impl Normalize for Quartic {
    fn normalize(&self) -> NormalizedQuartic {
        if self.a != 1.0 {
            NormalizedQuartic {
                b: self.b / self.a,
                c: self.c / self.a,
                d: self.d / self.a,
                e: self.e / self.a,
            }
        } else {
            NormalizedQuartic {
                b: self.b,
                c: self.c,
                d: self.c,
                e: self.e,
            }
        }
    }
}
//normalizing:

pub trait DepressedFormulas {
    fn p(&self) -> f64;
    fn r(&self) -> f64;
    fn q(&self) -> f64;
    fn ferraris_cubic_or_biquadratic(&self) -> Equations;
}

impl DepressedFormulas for NormalizedQuartic {
    fn p(&self) -> f64 {
        let second_term = (3.0 * self.b.powi(2)) / 8.0;
        self.c - second_term
    }

    fn r(&self) -> f64 {
        let second_term = (self.b * self.d) / 4.0;
        let third_term = (self.b.powi(2) * self.c) / 16.0;
        let fourth_term = (3.0 * self.b.powi(4)) / 256.0;
        self.e - second_term + third_term - fourth_term
    }

    fn q(&self) -> f64 {
        let second_term = self.b * self.c / 2.0;
        let third_term = self.b.powi(3) / 8.0;
        self.d - second_term + third_term
    }

    fn ferraris_cubic_or_biquadratic(&self) -> Equations {
        let p = self.p();
        let q = self.q();
        let r = self.r();
        if q != 0.0 {
            //if the  depressed quartic equation isn't biquadratic
            let degree_2_coefficient = -(p / 2.0);
            let degree_1_coefficient = -r;
            let degree_0_coefficient = (p * r / 2.0) - (q.powi(2) / 8.0);
            Equations::Cubic(transformer::Cubiceqn {
                a: 1.0,
                b: degree_2_coefficient,
                c: degree_1_coefficient,
                d: degree_0_coefficient,
            })
        } else {
            //if it is biquadratic
            Equations::Biquadratic(general::BiquadraticDegree4 { a: 1.0, b: p, c: r })
        }
    }
}
