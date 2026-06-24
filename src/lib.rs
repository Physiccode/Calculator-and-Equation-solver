//tests file
mod cubic_equation_handler;
mod utils;

use cubic_equation_handler::{rootscubic, solver, transformer};
use rootscubic::*;
use solver::*;
use utils::{algebra, geometry, miscellaneous, trig};

#[cfg(test)]
mod tests {
    use super::*;
    //testing algebra functions:

    #[test]
    fn test_factorial() {
        assert_eq!(algebra::factorial(5), 120);
        assert_eq!(algebra::factorial(0), 1);
    }

    #[test]
    fn test_power() {
        assert_eq!(algebra::power(2.0, 3.0), 8.0);
        assert_eq!(algebra::power(5.0, 0.0), 1.0);
    }

    //testing the third degree equation solver:
    #[test]
    fn test_roots() {
        let equation_1 = transformer::Cubiceqn {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 0.0,
        };
        let root = rootscubic::Root::Real(0.0).extract();
        let (r1, r2, r3) = equation_1.roots();

        assert_eq!(
            (&r1.extract(), &r2.extract(), &r3.extract()),
            (&root, &root, &root)
        ); //for  x³ = 0
    }
}
