use super::rootsquartic::QuarticRoots;
use super::rootsquartic::{get_back_x, get_u, get_v, quadratics};
use super::transformer::{DepressedFormulas, Equations, Normalize, Quartic};
use crate::cubic_equation_handler::{rootscubic::Root, transformer::Cubiceqn}; //load complex roots support
//self for callingroots

pub trait SolveQuartic {
    fn roots(&self) -> QuarticRoots;

    fn x1(&self) -> Root;
    fn x2(&self) -> Root;
    fn x3(&self) -> Root;
    fn x4(&self) -> Root;
}

impl SolveQuartic for Quartic {
    fn roots(&self) -> QuarticRoots {
        let normalized_quartic = self.normalize(); //normalize
        let q = normalized_quartic.q(); //initialize q
        let p = normalized_quartic.p(); //initialize p
        let b = &normalized_quartic.b;
        let ferraris_cubic_or_biquadratic = normalized_quartic.ferrarris_cubic_or_biquadratic();
        match ferraris_cubic_or_biquadratic {
            Equations::Cubic(Cubiceqn { a, b, c, d }) => {
                //if it is a cubic equation,then its the ferrari's cubic,we'll solve it the traditional way
                let real_root;
                //step 1:get the roots,lets call them z1,z2 and z3
                let (z_1, z_2, z_3) = ferraris_cubic_or_biquadratic.roots();
                //step 2,look for a real root
                for root in [z_1, z_2, z_3] {
                    loop {
                        match root {
                            Root::Real(content) => {
                                real_root = content;
                                break;
                            }
                            _ => continue,
                        }
                    }
                }

                //step 3:get u and v to form the 2 auxilliary quadratics
                let u = get_u(&real_root, &p);
                let v = get_v(&u, &q)?;
                let (depressed_quadratic_1, depressed_quadratic_2) = quadratics(&u, &real_root, &v);

                //step 4:solve for each quadratic to get the depressed roots
                let (y_1, y_2) = depressed_quadratic_1.roots()?;
                let (y_3, y_4) = depressed_quadratic_2.roots()?;

                //step 5:solve for the roots of the original equations
                let x_1 = get_back_x(y_1, &b);
                let x_2 = get_back_x(y_2, &b);
                let x_3 = get_back_x(y_3, &b);
                let x_4 = get_back_x(y_4, &b);
                QuarticRoots { x_1, x_2, x_3, x_4 }
            }

            Equations::Biquadratic(BiquadraticDegree4 { a, b, c }) => {
                //solve for the biquadratic depressed roots
                let (y_1, y_2, y_3, y_4) = self.biroots();

                //get back x:
                let x_1 = get_back_x(y_1, &b);
                let x_2 = get_back_x(y_2, &b);
                let x_3 = get_back_x(y_3, &b);
                let x_4 = get_back_x(y_4, &b);
                QuarticRoots { x_1, x_2, x_3, x_4 }
            }
        }
    }

    fn x1(&self) -> Root {
        return self.roots().x_1;
    }
    fn x2(&self) -> Root {
        return self.roots().x_2;
    }
    fn x3(&self) -> Root {
        return self.roots().x_3;
    }
    fn x4(&self) -> Root {
        return self.roots().x_4;
    }
}
