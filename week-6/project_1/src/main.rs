use std::io;
fn trapezium() {
    println!("\nThis is the Area of Trapezium Calculator.");

    //getting inputs
    println!("\nHeight:");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Not a valid string");
    let height:f32 = height.trim().parse().expect("Please enter a valid number");

    println!("\nBase 1:");
    let mut base1 = String::new();
    io::stdin().read_line(&mut base1).expect("Not a valid string");
    let base1:f32 = base1.trim().parse().expect("Please enter a valid number");

    println!("\nBase 2:");
    let mut base2 = String::new();
    io::stdin().read_line(&mut base2).expect("Not a valid string");
    let base2:f32 = base2.trim().parse().expect("Please enter a valid number");

    //calculating the area
    let area:f32 = (height/2.0)*(base1 + base2);
    println!("\nThe area is {}", area);
    println!("Thank you for using this calculator");
    println!("\n----------------------------------------------");

    main();


}
fn rhombus() {
    println!("\nThis is the Area of Rhombus Calculator.");
    
    //getting inputs
    println!("\nDiagonal 1:");
    let mut diag1 = String::new();
    io::stdin().read_line(&mut diag1).expect("Not a valid string");
    let diag1:f32 = diag1.trim().parse().expect("Please enter a valid number");

    println!("\nDiagonal 2:");
    let mut diag2 = String::new();
    io::stdin().read_line(&mut diag2).expect("Not a valid string");
    let diag2:f32 = diag2.trim().parse().expect("Please enter a valid number");

    //calculating the area
    let area:f32 = (diag1 * diag2)/2.0;
    println!("\nThe area is {}", area);
    println!("Thank you for using this calculator");
    println!("\n----------------------------------------------");

    main();
}
fn parallelogram() {
    println!("\nThis is the Area of Parallelogram Calculator.");
    
    //getting inputs
    println!("\nBase :");
    let mut base = String::new();
    io::stdin().read_line(&mut base).expect("Not a valid string");
    let base:f32 = base.trim().parse().expect("Please enter a valid number");

    println!("\nAltitude:");
    let mut alt = String::new();
    io::stdin().read_line(&mut alt).expect("Not a valid string");
    let alt:f32 = alt.trim().parse().expect("Please enter a valid number");

    //calculating the area
    let area:f32 = base * alt;
    println!("\nThe area is {}", area);
    println!("Thank you for using this calculator");
    println!("\n----------------------------------------------");

    main();
}
fn cube() {
    println!("\nThis is the Area of Cube Calculator.");
    
    //getting inputs
    println!("\nLength of the side :");
    let mut length = String::new();
    io::stdin().read_line(&mut length).expect("Not a valid string");
    let length:f32 = length.trim().parse().expect("Please enter a valid number");

    //calculating the area
    let area:f32 = (length.powf(2.0)) * 6.0;
    println!("\nThe area is {}", area);
    println!("Thank you for using this calculator");
    println!("\n----------------------------------------------");

    main();
}
fn cylinder() {
    println!("\nThis is the Area of Cylinder Calculator.");
    
    //getting inputs
    println!("\nHeight :");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Not a valid string");
    let height:f32 = height.trim().parse().expect("Please enter a valid number");

    println!("\nRadius:");
    let mut rad = String::new();
    io::stdin().read_line(&mut rad).expect("Not a valid string");
    let rad:f32 = rad.trim().parse().expect("Please enter a valid number");

    //calculating the area
    let area:f32 = 3.14 * (rad.powf(2.0)) * height;
    println!("The area is {}", area);
    println!("Thank you for using this calculator");
    println!("\n----------------------------------------------");
    
    main();

}


fn main() {
    
    println!("\nThis is the Shape Area Calculator ");
    println!("\nThese are all our calculators:");
    println!("1 - Area of Trapezium Calculator, \n2 - Area of Rhombus Calculator, \n3 - Area of Parallelogram Calculator, \n4 - Area of Cube Calculator, \n5 - Area of Cylinder Calculator");
    
    println!("\nWhat Calculator would you like to use?(type the corresponding number of the calculator. ie. 4 for cube). Press 0 to Quit.");
    let mut calculator = String::new();
    io::stdin().read_line(&mut calculator).expect("Not a valid string");
    let calculator:i32 = calculator.trim().parse().expect("Please enter a valid number");

    println!("\n----------------------------------------------");

    //If statements to call the corresponding functions
    if calculator == 1 {
       trapezium();
    } else if calculator == 2 {
        rhombus();
    } else if calculator == 3 {
        parallelogram();
    } else if calculator == 4 {
        cube();
    } else if calculator == 5 {
        cylinder();
    } else if calculator == 0{
        return
    }
    else {
        println!("Invalid selection, please try again.");
        
        main();
    } 
}
