use std::io;
pub fn print_modes() {
    //this shall print the principal modes
    println!(
        "
        Select the desired mode:
        0  - exit
        1  - Fourth degree equation solver [included complex roots]
        2  - Third degree equation solver  [included complex roots]
        3  - Second degree equation solver [included complex roots]
        4  - System of equations solver
        5  - Area calculations
        6  - Addition
        7  - Subtraction
        8  - Multiplication
        9  - Division
        10  - Floor division
        11 - Power
        12 - Radians to degrees
        13 - Degrees to radians
        14 - Functions operations
            "
    );
}

pub fn print_modes_secondary() {
    //This shall print modes when inside mode 4
    println!(
        "Select the desired calculation:
        1 - Area of a square
        2 - Area of a circle
        3 - Area of  rectangle
        4 - Area of right angled triangle
        5 - Area of other type of triangle
        --perimeters--
        a - perimeter of square
        b - perimeter of circle
        c - perimter of rectangle
        d - perimeter of right angled triangle
        e - perimeter of other type of triangle
        "
    );
}
//add:quadratic function points support,linear function,exponential function,etc...
pub fn print_objective() {
    println!(
        "Select the desired mode:
            1 - go back
            2 - Plotting ranges of x in y=mx+c
            3 - plotting ranges of y in y=mx+c
            4 - finding x and y axis in y=mx+c
            "
    );
}
pub fn readvar() -> String {
    let mut var = String::new();
    io::stdin().read_line(&mut var).expect("Couldnt read line!");
    var
}
