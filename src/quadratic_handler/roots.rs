pub struct QuadraticEquation {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

pub enum Root {
    Real(f64),
    Complex { re: f64, im: f64 },
}

pub trait Solver {
    fn discriminant(&self) -> f64;
    fn Roots(&self) -> (Result<Root, String>, Result<Root, String>); //return a result becouse a may be 0, which causes anomalies
}

impl Solver for QuadraticEquation {
    fn discriminant(&self) -> f64 {
        return self.b.powi(2) - (4 * self.a * self.c);
    }

    fn Roots(&self) -> (Result<Root, String>, Result<Root, String>) {
        if self.a != 0.0 {
            let discriminant_root = self.discriminant().sqrt();
            let eq = (-b / 2 * self.a);
            let positive_sum = eq + (discriminant_root / 2 * self.a);
            let negative_sum = eq - (discriminant_root / 2 * self.a);
            (Ok(positive_root), Ok(negative_root))
        } else {
            Err("coeficient of second degree must not be 0");
        }
    }
}
