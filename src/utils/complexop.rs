use crate::cubic_equation_handler::roots; //complex roots

pub trait Operations {
    fn cmplxsqrt(&self) -> (roots::Root, roots::Root);
}

impl Operations for roots::Root {
    fn cmplxsqrt(&self) -> (roots::Root, roots::Root) {
        // let the square root of a complex number be x+yi,then:
        match self {
            roots::Root::Complex { re, im } => {
                let a = re;
                let b = im;
                let x = ((a + (a.powi(2) + b.powi(2)).sqrt()) / 2.0).sqrt();
                let y = ((-a + (a.powi(2) + b.powi(2)).sqrt()) / 2.0).sqrt();

                let firstsolution = roots::Root::Complex { re: x, im: y };
                let secondsolution = roots::Root::Complex { re: -x, im: -y };

                return (firstsolution, secondsolution);
            }
            _ => panic!("function does not operate on real numbers"),
        }
    }
}
