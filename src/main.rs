use std::error::Error;

mod cubic_equation_handler;
mod utils; //Load my utils folder //Load the cubic equation handler folder

use algebra::*;
use geometry::*;
use miscellaneous::*;
use trig::*;
use utils::{algebra, geometry, miscellaneous, trig}; //load the content inside my utils folder

use cubic_equation_handler::{roots, solver, transformer};
use roots::*;
use solver::*;
use transformer::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Do you wish to break after each answer[yes/no]?:");
    let brake = readvar();

    'program_loop: loop {
        print_modes(); //display modes to screen
        let exit = readvar();

        if exit.trim() == "exit" || exit.trim() == "0" {
            break;
        } else if exit.trim() == "1" {
            println!("Mode:1-[Third degree equation solver]");

            println!("Enter the value of a:");
            let a: f64 = readvar().trim().parse::<f64>()?;
            println!("Enter the value of b:");
            let b: f64 = readvar().trim().parse::<f64>()?; //taking input
            println!("Enter the value of c:");
            let c: f64 = readvar().trim().parse::<f64>()?;
            println!("Enter the value of d:");
            let d: f64 = readvar().trim().parse::<f64>()?;

            let eqn = Cubiceqn { a, b, c, d }; //construct equation
            let (x1, x2, x3) = eqn.roots();
            let (discriminant, p, q, angle) = eqn.info();
            println!(
                "
                Result:
                1st root x={}
                2nd root x={}
                3rd root x={}
                Additional information:
                p from normalized equation:{}
                q from normalized equation:{}
                discriminant's value:{}
                angle whose cosine matches the cubic's parameters:{}

                ",
                x1.extract(),
                x2.extract(),
                x3.extract(),
                p,
                q,
                discriminant,
                angle
            );
        } else if exit.trim() == "2" {
            println!("Mode:2-[Second degree equation solver]");

            println!("Whats the value of a in ax^2 :");
            let a: f64 = readvar().trim().parse::<f64>()?;
            println!("Whats the value of a in bx :");
            let b: f64 = readvar().trim().parse::<f64>()?;
            println!("Whats the value of a in c :");
            let c: f64 = readvar().trim().parse::<f64>()?;

            let delta = secdeg_delta(a, b, c);

            println!("Do you want to calculate fractions? :");
            let calcfrac = readvar();

            if calcfrac.trim() == "yes" {
                let posroot = secdeg_posroot(a, b, &delta, true);
                let negroot = secdeg_negroot(a, b, &delta, true);
                println!(
                    "
                    Delta:{}
                    1st root(sum of square root):{}
                    2nd root(subtraction of square root):{}
                    Equation:[{}]x²+[{}]x+[{}]=0
                        ",
                    delta, posroot, negroot, a, b, c
                );
                if brake.trim() == "yes" {
                    break;
                }
            } else if calcfrac.trim() == "no" {
                let posroot = algebra::secdeg_posroot(a, b, &delta, false);
                let negroot = algebra::secdeg_negroot(a, b, &delta, false);
                println!(
                    "
                    Delta:{}
                    1st root(sum of square root):{}
                    2nd root(subtraction of square root):{}
                    Equation:[{}]x²+[{}]x+[{}]=0
                        ",
                    delta, posroot, negroot, a, b, c
                );
                if brake.trim() == "yes" {
                    break;
                }
            }
        } else if exit.trim() == "3" {
            println!("a_1 as in a1x:");
            let a_1: f64 = readvar().trim().parse::<f64>()?;
            println!("a_2 as in a2x:");
            let a_2: f64 = readvar().trim().parse::<f64>()?;
            println!("b_1 as in b1x:");
            let b_1: f64 = readvar().trim().parse::<f64>()?;
            println!("b_2 as in b2x:");
            let b_2: f64 = readvar().trim().parse::<f64>()?;
            println!("c_1 as in c1:");
            let c_1: f64 = readvar().trim().parse::<f64>()?;
            println!("c_2 as in c2:");
            let c_2: f64 = readvar().trim().parse::<f64>()?;

            let delta = twosys_delta(&a_1, &b_1, &a_2, &b_2);
            let deltax = twosys_deltax(&c_1, &b_1, &c_2, &b_2);
            let deltay = twosys_deltay(&a_1, &a_2, &c_1, &c_2);
            let calcx = twosys_calcdeltx(&deltax, &delta);
            let calcy = twosys_calcdelty(&deltay, &delta);

            classify(
                deltax, deltay, delta, calcx, calcy, a_1, b_1, a_2, b_2, c_1, c_2,
            );
            if brake.trim() == "yes" {
                break;
            }
        } else if exit.trim() == "4" {
            print_modes_secondary();
            let choice = readvar();

            //Choices within choice 4

            if choice.trim() == "1" {
                println!("What is the measurement of one side? :");
                let stside = readvar().trim().parse::<f64>()?;

                let square = Shape::Square {
                    side: stside.clone(),
                };
                let res = square.area(&square)?;

                println!(
                    "Dimensions of square:({}) x ({})
                      Area:{}",
                    &stside, stside, res
                );

                if brake.trim() == "yes" {
                    break;
                }
            } else if choice.trim() == "2" {
                println!("Insert the radius of the circle? :");
                let radius: f64 = readvar().trim().parse::<f64>()?;

                let circle = Shape::Circle { radius: radius };
                let res = circle.area(&circle)?;

                println!(
                    "Circle's radius:{}\nCircle's Area:{}\nCircle's area formula:pi*(radius^2)",
                    radius, res
                );

                if brake.trim() == "yes" {
                    break;
                }
            } else if choice.trim() == "3" {
                println!("What is the width of the rectangle? :");
                let width: f64 = readvar().trim().parse::<f64>()?;
                println!("Whats the length of the rectangle? :");
                let length: f64 = readvar().trim().parse::<f64>()?;

                let rect = Shape::Rectangle {
                    base: width,
                    height: length,
                };
                let res = rect.area(&rect)?;

                println!(
                    "Rectangles length:{}\nRectangle's width:{}\nRectangle's Area:{}",
                    length.clone(),
                    width.clone(),
                    res
                );

                if brake.trim() == "yes" {
                    break;
                }
            } else if choice.trim() == "4" {
                println!("[4-4] Mode:Área of right angled triangle");

                println!("Whats the measurement of the base? :");
                let base = readvar().trim().parse::<f64>()?;
                println!("Whats the measurement of the height? :");
                let height = readvar().trim().parse::<f64>()?;
                println!("Whats the measurement of the hypothenuse? :");
                let hypothenuse = readvar().trim().parse::<f64>()?;

                //construct triangle
                let rightangledtriangle = Shape::RightAngledTriangle {
                    base: base.clone(),
                    height: height.clone(),
                    hypothenuse: hypothenuse.clone(),
                };
                let res = rightangledtriangle.area(&rightangledtriangle)?;
                let classifside = rightangledtriangle.detect_type_by_sides(&rightangledtriangle)?;

                println!(
                    "
            angle classification: Right angled
            side classification: {}
            triangle's base:{}
            triangle'height:{}
            triangle's hypothenuse:{}
            triangle's area:{}
            ",
                    classifside, base, height, hypothenuse, res
                );
            } else if choice.trim() == "5" {
                println!("[4-5] Mode:Área of other type of triangle");

                println!("Whats the measurement of side a:");
                let a = readvar().trim().parse::<f64>()?;
                println!("Whats the measurement of side b:");
                let b = readvar().trim().parse::<f64>()?;
                println!("whats the measurement of side c:");
                let c = readvar().trim().parse::<f64>()?;

                let triangle = Shape::OtherTypeTriangle {
                    a: a.clone(),
                    b: b.clone(),
                    c: c.clone(),
                };

                let res = triangle.area(&triangle)?;
                let classifside = triangle.detect_type_by_sides(&triangle)?;
                let angleclassif = triangle.detect_type_by_angles(&triangle);

                println!(
                    "
            angle classification:{}
            side classification: {}
            triangle's side a:{}
            triangle's side b:{}
            triangle's side c:{}
            triangle's area:{}
            ",
                    angleclassif, classifside, a, b, c, res
                );
            }

            //choices outside choice 4
        } else if exit.trim() == "5" {
            println!("Select the value of x:");
            let x = readvar().trim().parse::<f64>()?;
            println!("Select the value of y:");
            let y = readvar().trim().parse::<f64>()?;

            println!("[{}]+[{}]={}", x.clone(), y.clone(), add(x, y));

            if brake.trim() == "yes" {
                break;
            }
        } else if exit.trim() == "6" {
            println!("Select the value of x:");
            let x = readvar().trim().parse::<f64>()?;
            println!("Select the value of y:");
            let y = readvar().trim().parse::<f64>()?;

            println!("[{}]-[{}]={}", x.clone(), y.clone(), subtract(x, y));

            if brake.trim() == "yes" {
                break;
            }
        } else if exit.trim() == "7" {
            println!("Select the value of x:");
            let x: f64 = readvar().trim().parse::<f64>()?;
            println!("Select the value of y:");
            let y: f64 = readvar().trim().parse::<f64>()?;

            multiply(x, y);
            if brake.trim() == "yes" {
                break;
            }
        } else if exit.trim() == "8" {
            println!("Select the value of x:");
            let x: f64 = readvar().trim().parse::<f64>()?;
            println!("Select the value of y:");
            let y: f64 = readvar().trim().parse::<f64>()?;

            divide(x, y);
            if brake.trim() == "yes" {
                break;
            }
        } else if exit.trim() == "9" {
            println!("Select the value of x:");
            let x: f64 = readvar().trim().parse::<f64>()?;
            println!("Select the value of y:");
            let y: f64 = readvar().trim().parse::<f64>()?;

            floordivide(x, y);

            if brake.trim() == "yes" {
                break;
            }
        } else if exit.trim() == "10" {
            println!("Select the value of x:");
            let x: f64 = readvar().trim().parse::<f64>()?;
            println!("Select the value of r:");
            let r: f64 = readvar().trim().parse::<f64>()?;

            println!("[{}]**[{}]={}", x.clone(), r.clone(), power(x, r));

            if brake.trim() == "yes" {
                break;
            }
        } else if exit.trim() == "11" {
            println!("[11] Mode:Radians to degrees");
            println!(
                "How many radians do you wish to convert to degrees? enter them in the form of '2pi/3' :"
            );
            let rad = readvar();
            let res = radtodeg(rad.clone())?;
            println!("{} rad -> {} degrees", rad.trim(), res);
            if brake.trim() == "yes" {
                break;
            }
        } else if exit.trim() == "12" {
            println!("[12] Mode:Degrees to radians");

            println!(
                "How many degrees do you wish to convert to radians? enter them in the form of '150' :"
            );
            let deg: f64 = readvar().trim().parse::<f64>()?;
            println!("Do you want to calculate fractions? :");
            let calcfrac = readvar();

            //in the future make it so user cna input 150º
            if calcfrac.trim() == "no" {
                let rad = degreestorad(&deg, true);
                println!("[{}] degrees-->[{}] radians", deg, rad);
            } else if calcfrac.trim() == "no" {
                let rad = degreestorad(&deg, false);
                println!("[{}] degrees-->[{}] radians", deg, rad);
            }
        } else if exit.trim() == "13" {
            println!("[12] Mode:Functions");

            loop {
                print_objective();

                let funcchoice = readvar();

                if funcchoice.trim() == "1" {
                    break;
                } else if funcchoice.trim() == "2" {
                    println!("Enter value of m:");
                    let mval: f64 = readvar().trim().parse::<f64>()?;
                    println!("Enter value of c:");
                    let cval: f64 = readvar().trim().parse::<f64>()?;
                    println!("Enter the first value of x :");
                    let mut stval: f64 = readvar().trim().parse::<f64>()?;
                    println!("Enter the second value of x :");
                    let ndval: f64 = readvar().trim().parse::<f64>()?;
                    let mut valsy: Vec<f64> = Vec::new();

                    while stval < ndval {
                        let i = plotx(mval, cval, stval);
                        valsy.push(i);
                        stval += 1.0;
                    }
                    println!(
                        "Equation:y=({})x+({}) for x in range=[({}),({})]",
                        mval, cval, stval, ndval
                    );
                    println!("Values of y:{:#?}", valsy);
                    if brake.trim() == "yes" {
                        break 'program_loop;
                    }
                } else if funcchoice.trim() == "3" {
                    let mut valsx: Vec<String> = Vec::new();
                    println!("Enter value of m:");
                    let mval: f64 = readvar().trim().parse::<f64>()?;
                    println!("Enter value of c:");
                    let cval: f64 = readvar().trim().parse::<f64>()?;
                    println!("Enter the first value of y:");
                    let mut stval: f64 = readvar().trim().parse::<f64>()?;
                    println!("Enter the last value of y:");
                    let ndval: f64 = readvar().trim().parse::<f64>()?;
                    println!("Do you want to calculate fraction? :");
                    let calcfrac = readvar();

                    while stval < ndval {
                        if calcfrac.trim() == "yes"
                            || calcfrac.trim() == "calculate"
                            || calcfrac.trim() == "accept"
                        {
                            let i = ploty(mval, cval, stval, true);
                            valsx.push(i);
                            stval += 1.0;
                        } else {
                            let i = ploty(mval, cval, stval, false);
                            valsx.push(i);
                            stval += 1.0;
                        }
                    }

                    println!(
                        "Equation:y={}x+{} for y in range=[{},{}]",
                        mval, cval, stval, ndval
                    );
                    println!("Values of y as x varies:{:#?}", valsx);
                    if brake.trim() == "yes" {
                        break;
                    }
                }
            }
        } else {
            println!("Invalid choice");
            if brake.trim() == "yes" {
                break;
            }
        }
    }
    Ok(())
}
