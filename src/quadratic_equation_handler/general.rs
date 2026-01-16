use crate::cubic_equation_handler::roots; //call real or complex roots enum
pub struct Quadratic {
    a:f64,
    b:f64,
    c:f64
}

pub trait Solve {
    fn discriminant(&self)->f64;
    fn roots(&self)->(Result<roots::Root,String>,Result<roots::Root,String>);
}

impl Solve for Quadratic {
    fn discriminant(&self)->f64 {
        let second_term = -(4*self.a*self.c);
        self.b.powi(2)+second_term
    }

    fn roots(&self) -> (Result<roots::Root,String>,Result<roots::Root,String>) {
        if determinant >= 0 {
            let first_term = -self.b/2.0*self.a;
            let sum = (self.b.powi(2)-4*self.a*self.c)/2*self.a;
            let r_1 = roots::Root::real(first_term + sum);
            let r_2 = roots::Root::real(first_term - sum);
            (Ok(r_1),Ok(r_2)
        }
        else {
            if self.a != 0 {

            }
        }

    }

}
