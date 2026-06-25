use super::super::cubic_equation_handler::rootscubic::Root; //complex roots

pub trait Operations {
    fn cmplxsqrt(&self) -> (Root, Root);
}

impl Operations for Root {
    fn cmplxsqrt(&self) -> (Root, Root) {
        // let the square root of a complex number be x+yi,then:
        match self {
            Root::Complex { re, im } => {
                let r = (re.powi(2) + im.powi(2)).sqrt();
                let real = ((r + re) / 2.0).sqrt();
                let imaginary = ((r - re) / 2.0).sqrt();
                let imaginary = if *im < 0.0 { -imaginary } else { imaginary };

                let firstsolution = Root::Complex {
                    re: real,
                    im: imaginary,
                };
                let secondsolution = Root::Complex {
                    re: -real,
                    im: -imaginary,
                };

                return (firstsolution, secondsolution);
            }
            _ => panic!("function does not operate on real numbers"),
        }
    }
}
