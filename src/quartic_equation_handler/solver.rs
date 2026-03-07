use crate::cubic_equation_handler::{roots::Root, solver::Solve, transformer::Cubiceqn};
use crate::quadratic_equation_handler::general::BiquadraticDegree4;
use crate::quartic_equation_handler::{self, solver, transformer}; //self for callingroots

pub trait Solve {
    fn roots(&self) -> (Root, Root, Root, Root);

    fn x1(&self) -> Root;
    fn x2(&self) -> Root;
    fn x3(&self) -> Root;
    fn x4(&self) -> Root;
}

impl Solve for transformer::Quartic {
    fn roots(&self) -> (Root, Root, Root, Root) {
        //normalize
        let normalized_quartic = self.normalize();
        let ferraris_cubic_or_biquadratic = normalized_quartic.ferrarris_cubic_or_biquadratic();
        match ferraris_cubic_or_biquadratic {
            Cubiceqn { a, b, c, d } => {
                //if it is a cubic equation,then its the ferrari's cubic,we'll solve it the traditional way
                let real_roots: Vec<Root> = vec![];
                //step 1:get the roots,lets call them z1,z2 and z3
                let (z_1, z_2, z_3) = ferraris_cubic_or_biquadratic.roots();
                //step 2,look for a real root
                for root in [z_1, z_2, z_3] {
                    while True {
                        match root {
                            Root::Real(content) => {
                                real_roots.push(root);
                                break;
                            }
                            _ => continue,
                        }
                    }
                }

                //step 3:form the 2 depressed equations
            }
            BiquadraticDegree4 { a, b, c } => {}
        }
    }
}
