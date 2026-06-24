use super::super::cubic_equation_handler::rootscubic::Root; //complex roots

pub trait Operations {
    fn cmplxsqrt(&self) -> (Root, Root);
}

impl Operations for Root {
    fn cmplxsqrt(&self) -> (Root, Root) {
        // let the square root of a complex number be x+yi,then:
        match self {
            Root::Complex { re, im } => {
                let a = re;
                let b = im;
                let x = ((a + (a.powi(2) + b.powi(2)).sqrt()) / 2.0).sqrt();
                let y = ((-a + (a.powi(2) + b.powi(2)).sqrt()) / 2.0).sqrt();

                let firstsolution = Root::Complex { re: x, im: y };
                let secondsolution = Root::Complex { re: -x, im: -y };

                return (firstsolution, secondsolution);
            }
            _ => panic!("function does not operate on real numbers"),
        }
    }
}
