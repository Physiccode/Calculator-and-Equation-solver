use crate::cubic_equation_handler::transformer;

pub struct Quartic {
    pub a:f64,
    pub b:f64,
    pub c:f64,
    pub d:f64,
    pub e:f64
}

pub struct NormalizedQuartic { // A = 1
    pub b:f64,
    pub c:f64,
    pub d:f64,
    pub e:f64
}
//normalizing:
fn normalize(any:&Quartic) -> NormalizedQuartic {
    if any.a != 1.0 {
        NormalizedQuartic{
            b:any.b/any.a,
            c:any.c/any.a,
            d:any.d/any.a,
            e:any.e/any.a
        }
    }
    NormalizedQuartic {
        b:any.b,
        c:any.c,
        d:any.c,
        e:any.e
    }
}

pub trait DepressedFormulas {
    fn p(&self) -> f64;
    fn r(&self) -> f64;
    fn q(&self) -> f64;
    fn ferrarris_cubic(&self) -> transformer::Cubiceqn;
}

impl DepressedFormulas for NormalizedQuartic {
    fn p(&self) -> f64 {
        let second_term =(3*self.b.powi(2))/8.0;
        self.c-second_term
    }

    fn r(&self) -> f64 {
        let second_term = (3*self.b.powi(4))/256.0;
        let third_term =  (self.b.powi(2)*self.c)/16.0;
        let fourth_term = (self.b*self.d)/4.0;
        self.e - second_term + third_term - fourth_term
    }

    fn q(&self) -> f64 {
        let second_term = self.b.powi(3)/8.0;
        let third_term = self.b*self.c/2.0;
        self.d + second_term - third_term
    }

    fn ferraris_cubic(&self) {
        let p = self.p();
        let q = self.q();
        let r = self.r();
        let degree_2_coefficient = -(p/2.0);
        let degree_1_coefficient = -r;
        let degree_0_coefficient = (p*r/2.0)-(q.powi(2)/8.0);
    }
}
