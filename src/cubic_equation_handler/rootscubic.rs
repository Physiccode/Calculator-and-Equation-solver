use crate::cubic_equation_handler::transformer::*;
use std::f64::consts::PI;

#[derive(Clone, Copy, Debug)]
pub enum Root {
    //Structure of a complex number:Z=a+bi,Re(z)=a,Im(z)=b
    Real(f64),
    Complex { re: f64, im: f64 },
}
pub struct DepressedRoots {
    pub y_1: Root,
    pub y_2: Root,
    pub y_3: Root,
    pub a: f64, //auxilliary
}
pub struct OriginalRoots {
    pub x_1: Root,
    pub x_2: Root,
    pub x_3: Root,
}
pub trait Roots {
    fn get_roots(&self) -> DepressedRoots;
}
pub trait Extract {
    fn extract(&self) -> String; //Parse the value so  it can be easily used in main
}

fn check_precision(x: f64) -> f64 {
    const EPS: f64 = 1e-8;

    if x > 0.0 {
        let decimal = x - x.floor(); //Take only decimal part

        if (1.0 - decimal) < EPS {
            return x.ceil(); //If the decimal part is very close to 1,we return ceil(x)
        } else {
            x
        } //otherwise,we just return x
    } else if x == 0.0 {
        x
    } else {
        //If x is negative,the function ceil and floor are used differently becouse of the real line
        let decimal = x - x.ceil(); //take only decimal part

        if (1.0 + decimal) < EPS { x.floor() } else { x } //otherwise,we just return x
    }
}

impl Extract for Root {
    fn extract(&self) -> String {
        const EPS: f64 = 1e-12;

        match self {
            Root::Real(value) => return value.to_string(),
            Root::Complex { re, im } => {
                if *re < EPS && *im < EPS {
                    format!("0")
                }
                //return just 0
                else if *re < EPS {
                    format!("{}i", im)
                }
                //Simplify the real part
                else if *im < EPS {
                    format!("{}", re)
                }
                // Simplify the imaginary part
                else {
                    if *im < 0.0 {
                        format!("{}-{}i", re, im.abs())
                    } else {
                        format!("{}+{}i", re, im)
                    }
                }
            }
        }
    }
}

impl Roots for DepressedCube {
    fn get_roots(&self) -> DepressedRoots {
        const EPS: f64 = 1e-12; //Numerical precision
        let delta = self.discriminant();

        if delta > 0.0 {
            //1 real root,2 complex conjugate roots

            let u = (-(self.q / 2.0) + self.discriminant().sqrt()).cbrt();
            let v = (-(self.q / 2.0) - self.discriminant().sqrt()).cbrt();
            let last_part: f64 = ((3 as f64).sqrt() / 2.0) * (u - v);
            let first_part: f64 = -((u + v) / 2.0);

            let y_1 = Root::Real(u + v);
            let y_2 = Root::Complex {
                re: first_part,
                im: last_part,
            };
            let y_3 = Root::Complex {
                re: first_part,
                im: -last_part,
            };

            DepressedRoots {
                y_1,
                y_2,
                y_3,
                a: self.a,
            }
        } else if delta.abs() < EPS {
            // if the discriminant is equal to 0:three real roots,at least two are equal
            if self.p == self.q && self.p == 0.0 {
                //if this condition is true,then all roots must be equal to 0
                let r = Root::Real(0.0);
                DepressedRoots {
                    y_1: r,
                    y_2: r,
                    y_3: r,
                    a: self.a,
                }
            } else {
                //then,2 roots are equal
                let u = (-self.q / 2.0).cbrt();

                DepressedRoots {
                    y_1: Root::Real(2.0 * u),
                    y_2: Root::Real(-u),
                    y_3: Root::Real(-u),
                    a: self.a,
                }
            }
        } else if delta < -EPS {
            //three distinct real roots,we must switch to trigonometry
            let first_part_w: f64 = 2.0 * (-(self.p / 3.0).powi(3)).sqrt();
            let first_part: f64 = 2.0 * (-self.p / 3.0).sqrt();
            let phi = (-self.q / first_part_w).clamp(-1.0, 1.0).acos(); //using clamp() to prevent out-of-domain

            let y_1 = Root::Real(first_part * (phi / 3.0).cos());
            let y_2 = Root::Real(first_part * ((phi + 2.0 * PI) / 3.0).cos());
            let y_3 = Root::Real(first_part * ((phi + 4.0 * PI) / 3.0).cos());

            DepressedRoots {
                y_1,
                y_2,
                y_3,
                a: self.a,
            }
        } else {
            panic!(
                "unknown error,perhaps,did you make sure you're following the correct cubic form\ndelta={}",
                delta
            );
        }
    }
}

pub trait Original {
    fn get_original_root_y_1(&self) -> Root;
    fn get_original_root_y_2(&self) -> Root;
    fn get_original_root_y_3(&self) -> Root;
    fn all_roots(&self) -> OriginalRoots;
}

impl Original for DepressedRoots {
    fn get_original_root_y_1(&self) -> Root {
        //plug back x
        match self.y_1 {
            Root::Real(r) => Root::Real(check_precision(r - (self.a / 3.0))),

            Root::Complex { re, im } => {
                let sum_of_reals = check_precision(-(self.a / 3.0) + re);

                Root::Complex {
                    //return
                    re: sum_of_reals,
                    im: check_precision(im),
                }
            }
        }
    }

    fn get_original_root_y_2(&self) -> Root {
        match self.y_2 {
            Root::Real(r) => Root::Real(check_precision(r - (self.a / 3.0))),

            Root::Complex { re, im } => {
                let sum_of_reals = check_precision(-(self.a / 3.0) + re);

                Root::Complex {
                    //return
                    re: sum_of_reals,
                    im: check_precision(im),
                }
            }
        }
    }

    fn get_original_root_y_3(&self) -> Root {
        match self.y_3 {
            Root::Real(r) => Root::Real(check_precision(r - (self.a / 3.0))),

            Root::Complex { re, im } => {
                let sum_of_reals = check_precision(-(self.a / 3.0) + re);

                Root::Complex {
                    //return
                    re: sum_of_reals,
                    im: check_precision(im),
                }
            }
        }
    }

    fn all_roots(&self) -> OriginalRoots {
        OriginalRoots {
            x_1: self.get_original_root_y_1(),
            x_2: self.get_original_root_y_2(),
            x_3: self.get_original_root_y_3(),
        }
    }
}
