use crate::cubic_equation_handler::rootscubic::*;
use crate::cubic_equation_handler::transformer::*;

pub trait Solve {
    //the principal trait,all the others in transformer and roots are not getting used in main or in tests
    fn roots(&self) -> (Root, Root, Root);
    fn x1(&self) -> String;
    fn x2(&self) -> String;
    fn x3(&self) -> String;
    fn info(&self) -> (String, String, String, String);
    //comming soon:sum of roots,product of roots,simplifyng the equation in other forms
}
impl Solve for Cubiceqn {
    fn roots(&self) -> (Root, Root, Root) {
        let r = self.normalize().depress().get_roots().all_roots();
        (r.x_1, r.x_2, r.x_3)
    }

    fn x1(&self) -> String {
        self.normalize()
            .depress()
            .get_roots()
            .all_roots()
            .x_1
            .extract()
    }

    fn x2(&self) -> String {
        self.normalize()
            .depress()
            .get_roots()
            .all_roots()
            .x_2
            .extract()
    }

    fn x3(&self) -> String {
        self.normalize()
            .depress()
            .get_roots()
            .all_roots()
            .x_3
            .extract()
    }
    fn info(&self) -> (String, String, String, String) {
        let discriminant = self.normalize().depress().discriminant().to_string();
        let p = self.normalize().p().to_string();
        let q = self.normalize().q().to_string();
        let short = self.normalize().depress();
        let angle = if short.discriminant() < 0.0 {
            (-short.q / (2.0 * (-(short.p / 3.0).powi(3)).sqrt()))
                .clamp(-1.0, 1.0)
                .acos()
                .to_string()
        } else {
            "None".to_string()
        };

        (discriminant, p, q, angle)
    }
}
