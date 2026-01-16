use crate::cubic_equation_handler::roots; //call real or complex roots enum
pub struct Quadratic {
    a:f64,
    b:f64,
    c:f64
}

pub trait Solve {
    fn discriminant(&self)->f64;
    fn roots(&self)->(roots::Root,roots::Root);
}
