use crate::cubic_equation_handler; //roots
use crate::quartic_equation_handler::roots;
use crate::quartic_equation_handler::solver;
use crate::quartic_equation_handler::transformer;

pub trait Solve {
    fn roots(
        &self,
    ) -> (
        cubic_equation_handler::roots::Root,
        cubic_equation_handler::roots::Root,
        cubic_equation_handler::roots::Root,
        cubic_equation_handler::roots::Root,
    );

    fn x1(&self) -> cubic_equation_handler::roots::Root;
    fn x2(&self) -> cubic_equation_handler::roots::Root;
    fn x3(&self) -> cubic_equation_handler::roots::Root;
    fn x4(&self) -> cubic_equation_handler::roots::Root;
}
