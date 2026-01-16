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
        let discriminant = self.discriminant();
        if determinant >= 0 {
            let first_term = -self.b/2.0*self.a;
            let sum = discriminant.sqrt()/2*self.a;
            let r_1 = roots::Root::Real(first_term + sum);
            let r_2 = roots::Root::Real(first_term - sum);
            (Ok(r_1),Ok(r_2)
        }
        else {
            if self.a != 0 {
                let first_term = -self.b/2.0*self.a;
                let sum = discriminant.abs().sqrt()/2*self.a;
                let r_1 = roots::Root::Complex{
                    re:first_term,
                    im:sum
                }
                let r_2 = roots::Root::Complex {
                    re: first_term,
                    im: -sum
                }
                (Ok(r_1),Ok(r_2))
            }
            Err("value of a can't be 0")
        }

    }

}
