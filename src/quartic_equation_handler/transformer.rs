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
    fn p(&self)->f64;
    fn r(&self)->f64;
    fn q(&self)->f64;
    fn ferrarris_cubic(&self)->transformer::Cubiceqn;
}
