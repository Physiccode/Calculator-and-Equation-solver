use crate::cubic_equation_handler::transformer;
use crate::quadratic_equation_handler::general;
use crate::quartic_equation_handler; //transformer //get the quadratic equation structure

/*Assuming you have a real solution formed by the ferrari's cubic,say,z.To solve it and get
the roots in y and in x,the code bellow  will show how */

pub fn get_u(z: f64, p: f64) -> f64 {
    first_term = (2 * z);
    return (first_term - p).sqrt();
}

//get_v depends on the value of u,which is obtained after running get_u()
pub fn get_v(q: f64, u: f64) -> f64 {}
