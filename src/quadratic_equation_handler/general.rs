use super::super::cubic_equation_handler::rootscubic::Root; //call real or complex roots enum
use crate::utils::complexop::Operations;

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

pub trait SolveQuadratic {
    fn discriminant(&self) -> f64;
    fn roots(&self) -> Result<(Root, Root), String>;
}

impl SolveQuadratic for Quadratic {
    fn discriminant(&self) -> f64 {
        let second_term = -(4.0 * self.a * self.c);
        self.b.powi(2) + second_term
    }

    fn roots(&self) -> (Result<(Root, Root), String>) {
        let discriminant = self.discriminant();
        if discriminant >= 0.0 {
            if self.a == 0.0 {
                return Err("Value of a can't be zero".to_string());
            } else {
                let first_term = -self.b / (2.0 * self.a);
                let sum = discriminant.sqrt() / (2.0 * self.a);
                let r_1 = Root::Real(first_term + sum);
                let r_2 = Root::Real(first_term - sum);
                return Ok((r_1, r_2));
            }
        } else {
            if self.a != 0.0 {
                let first_term = -self.b / 2.0 * self.a;
                let sum = discriminant.abs().sqrt() / (2.0 * self.a);
                let r_1 = Root::Complex {
                    re: first_term,
                    im: sum,
                };
                let r_2 = Root::Complex {
                    re: first_term,
                    im: -sum,
                };
                return Ok((r_1, r_2));
            } else {
                return Err("value of a can't be 0".to_string());
            }
        }
    }
}

pub trait SolveBiquadratic {
    fn discriminant(&self) -> f64;
    fn biroots(&self) -> Result<(Root, Root, Root, Root), String>;
}

impl SolveBiquadratic for BiquadraticDegree4 {
    fn discriminant(&self) -> f64 {
        let second_term = -(4.0 * self.a * self.c);
        self.b.powi(2) + second_term
    }

    fn biroots(&self) -> Result<(Root, Root, Root, Root), String> {
        let discriminant = self.discriminant();
        let first_term = -self.b / (2.0 * self.a);
        if self.a == 0.0 {
            return Err("Value of a can't be 0".to_string());
        }

        if discriminant >= 0.0 {
            let sum = discriminant.sqrt() / (2.0 * self.a);
            let r_1_squared = first_term + sum;
            let r_2_squared = first_term - sum;
            let r_1 = if r_1_squared >= 0.0 {
                Root::Real(r_1_squared.sqrt())
            } else {
                Root::Complex {
                    re: 0.0,
                    im: r_1_squared.abs().sqrt(),
                }
            };

            let r_2 = if r_1_squared >= 0.0 {
                Root::Real(-r_1_squared.sqrt())
            } else {
                Root::Complex {
                    re: 0.0,
                    im: -r_1_squared.abs().sqrt(),
                }
            };

            let r_3 = if r_2_squared >= 0.0 {
                Root::Real(r_2_squared.sqrt())
            } else {
                Root::Complex {
                    re: 0.0,
                    im: r_2_squared.abs().sqrt(),
                }
            };

            let r_4 = if r_2_squared >= 0.0 {
                Root::Real(-r_2_squared.sqrt())
            } else {
                Root::Complex {
                    re: 0.0,
                    im: -r_2_squared.abs().sqrt(),
                }
            };

            Ok((r_1, r_2, r_3, r_4))
        } else {
            let r_1_squared = Root::Complex {
                re: first_term,
                im: discriminant.abs().sqrt() / (2.0 * self.a),
            };

            let r_2_squared = Root::Complex {
                re: first_term,
                im: -discriminant.abs().sqrt() / (2.0 * self.a),
            };

            let (r_1, r_2) = r_1_squared.cmplxsqrt();
            let (r_3, r_4) = r_2_squared.cmplxsqrt();

            Ok((r_1, r_2, r_3, r_4))
        }
    }
}
