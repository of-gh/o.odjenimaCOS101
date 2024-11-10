use std::io;

fn main() {
    println!("Enter your first value (ax^2): ");
    let mut input1= String::new();
    io::stdin().read_line(&mut input1).expect("Failed to load input");
    let value_a:f32 = input1.trim().parse().expect("Failed to input");

    println!("Enter your second value (bx): ");
    let mut input2=String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let value_b:f32 = input2.trim().parse().expect("Failed to input");

    println!("Enter your third value (c): ");
    let mut input3=String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let value_c:f32 = input3.trim().parse().expect("Failed to input");
    
    //finding the discriminant
    let discriminant:f32 = (value_b.powf(2.0)) - 4.0*value_a*value_c;
    
    //creating conditions
    if discriminant < 0.0 {
        println!("There are no real roots, and your discriminant is {}", discriminant);
    } 
    else if discriminant > 0.0 {
        println!("There are two distinct roots, and your discriminant is {}", discriminant);
    }
    else if discriminant == 0.0 {
        println!("There is exactly one real root, and your discriminant is {}", discriminant);
    }

}
