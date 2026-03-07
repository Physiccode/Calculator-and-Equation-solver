use crate::cubic_equation_handler::roots::Root; //call real or complex roots enum
use crate::utils::complexop;

pub struct BiquadraticDegree4 {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}
pub struct Quadratic {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

pub trait Solve {
    fn discriminant(&self) -> f64;
    fn roots(&self) -> (Result<(Root, Root), String>);
}

impl Solve for Quadratic {
    fn discriminant(&self) -> f64 {
        let second_term = -(4 * self.a * self.c);
        self.b.powi(2) + second_term
    }

    fn roots(&self) -> (Result<(Root, Root), String>) {
        let discriminant = self.discriminant();
        if discriminant >= 0.0 {
            let first_term = -self.b / 2.0 * self.a;
            let sum = discriminant.sqrt() / 2 * self.a;
            let r_1 = Root::Real(first_term + sum);
            let r_2 = Root::Real(first_term - sum);
            return Ok(r_1, r_2);
        } else {
            if self.a != 0.0 {
                let first_term = -self.b / 2.0 * self.a;
                let sum = discriminant.abs().sqrt() / 2.0 * self.a;
                let r_1 = Root::Complex {
                    re: first_term,
                    im: sum,
                };
                let r_2 = Root::Complex {
                    re: first_term,
                    im: -sum,
                };
                return Ok(r_1, r_2);
            } else {
                return Err("value of a can't be 0".to_string());
            }
        }
    }
}

pub trait SolveBiquadratic {
    fn discriminant(&self) -> f64;
    fn roots(&self) -> Result<(Root, Root, Root, Root), String>;
}

impl SolveBiquadratic for BiquadraticDegree4 {
    fn discriminant(&self) -> f64 {
        let second_term = -(4 * self.a * self.c);
        self.b.powi(2) + second_term
    }

    fn roots(&self) -> Result<(Root, Root, Root, Root), String> {
        let discriminant = self.discriminant();
        if discriminant >= 0.0 {
            let first_term = -self.b / 2.0 * self.a;
            let sum = discriminant.sqrt() / 2.0 * self.a;
            let r_1_squared = first_term + sum;
            let r_2_squared = first_term - sum;
            if r_1_squared > 0.0 {
                let r_1 = Root::Real(r_1_squared.sqrt());
                let r_2 = Root::Real(-r_1_squared.sqrt());
                if r_2_squared > 0.0 {
                    let r_3 = Root::Real(r_2_squared.sqrt());
                    let r_4 = Root::Real(-r_2_squared.sqrt());
                } else {
                    let r_3 = Root::Complex {
                        re: 0.0,
                        im: r_2_squared.abs().sqrt(),
                    };
                    let r_4 = Root::Complex {
                        re: 0.0,
                        im: -r_2_squared.abs().sqrt(),
                    };
                }
            } else {
                let r_1 = Root::Complex {
                    re: 0.0,
                    im: r_1_squared.abs().sqrt(),
                };
                let r_2 = Root::Complex {
                    re: 0.0,
                    im: -r_1_squared.abs().sqrt(),
                };
            }
            return Ok(r_1, r_2, r_3, r_4);
        } else {
            if self.a != 0 {
                let real_part = -(self.b / 2.0 * self.a);
                let imaginary_part = (discriminant.abs().sqrt()) / (2.0 * self.a); //this part is the coefficient of i
                //now we have the squared roots,time to take the squareroot of them
                r_1_squared = Root::Complex {
                    re: real_part,
                    im: imaginary_part,
                };
                r_2_squared = Root::Complex {
                    re: real_part,
                    im: -imaginary_part,
                };
                let (r_1, r_2) = r_1_squared.cmplxsqrt();
                let (r_3, r_4) = r_2_squared.cmplxsqrt();
                return Ok(r_1, r_2, r_3, r_4);
            }
            Err("value of a can't be 0".to_string())
        }
    }
}
