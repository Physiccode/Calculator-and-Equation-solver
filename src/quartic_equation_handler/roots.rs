use crate::cubic_equation_handler::roots::Root;
use crate::quadratic_equation_handler::general;
use crate::quartic_equation_handler; //transformer //get the quadratic equation structure

pub struct QuarticRoots {
    x_1: Root,
    x_2: Root,
    x_3: Root,
    x_4: Root,
}

/*Assuming you have a real solution formed by the ferrari's cubic,say,z.To solve it and get
the roots in y and in x,the code bellow  will show how */

pub fn get_u(z: &f64, p: &f64) -> f64 {
    first_term = (2 * z);
    return (first_term - p).sqrt();
}

//get_v depends on the value of u,which is obtained after running get_u()
pub fn get_v(q: &f64, u: &f64) -> Result<f64, String> {
    if u != 0.0 {
        Ok(q / (2.0 * u))
    } else {
        Err("Value of u can't be 0,it causes division by zero".to_string())
    }
}

pub fn quadratics(u: &f64, z: &f64, v: &f64) -> (general::Quadratic, general::Quadratic) {
    //depressed quadratic factors
    let first_quadratic = general::Quadratic {
        a: 1.0,
        b: u,
        c: z - v,
    };

    let second_quadratic = general::Quadratic {
        a: 1.0,
        b: -u,
        c: z + v,
    };

    return (first_quadratic, second_quadratic);
}

pub fn get_back_x(y: roots::Root, b: f64) -> roots::Root {
    //b is the coefficient of the third degree element from the normalized quartic
    match y {
        roots::Root::Real(yroot) => {
            let x = y - (b / 4.0);
            return x;
        }
        roots::Root::Complex { re, im } => {
            let im = imaginary;
            let real = re - b / 4.0;
            let x = roots::Root::Complex {
                re: real,
                im: imaginary,
            };
        }
    }
}
