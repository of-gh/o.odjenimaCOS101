use std::io;

fn main() {

    println!("This is the Student Council Voter System. Welcome.");

    // Input of student details
    println!("\nWhat is your full name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to load input");
    
    println!("Please input your school email");
    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("Failed to load input");

    println!("What is your department");
    let mut department = String::new();
    io::stdin().read_line(&mut department).expect("Failed to load input");
    
    println!("What is your State of Origin");
    let mut soo = String::new();
    io::stdin().read_line(&mut soo).expect("Failed to load input");
    
    println!("\nThank you for making it this far.");
    
    //Class rep confirmation
    println!("\nIf you are a class rep type 1. Else, type 2 ");
    let mut input1=String::new();
    io::stdin().read_line(&mut input1).expect("Failed to load input");
    let classrep:i32 = input1.trim().parse().expect("Not a valid number");
    if classrep !=1 && classrep !=2{
        println!("Invalid input");
        std::process::exit(0);
    }
    if classrep == 2{
        println!("Sorry you are not eligible to vote")
        std::process::exit(0);
    }

    println!("What level are you in?");
    let mut input2=String::new();
    io::stdin().read_line(&mut input2).expect("Failed to load input");
    let level:i32 = input2.trim().parse().expect("Not a valid number");

    println!("What is your CGPA?");
    let mut input3=String::new();
    io::stdin().read_line(&mut input3).expect("Failed to load input");
    let cgpa:f32 = input2.trim().parse().expect("Not a valid number");
    
    //conditional statement to verify eligibility to vote
    if  level==100 && cgpa < 4.0 {
        println!("Sorry, you are not eligible to vote")
    } 
    else {println!("Full Name: {}, Email: {}, Department: {}, State Of Origin: {}.", name, email, department, soo);
   println!("You can vote");
   
   // count till 50
   let mut count = 0;
   loop{
       count+=1;
       if count ==51{
           break;
       }
   }
    }
 
}

// Question 2. Please Run Seperately.

use std::io;

fn main() {
    println!("Welcome to The Staff Publication Incentive System");
    
    //Input Name
    println!("\nWhat is the Staff's full name? ");
    let mut name= String::new();
    io::stdin().read_line(&mut name).expect("Failed to load input");

    //Staff publishing number
    println!("How many papers has the staff published? ");
    let mut input2= String::new();
    io::stdin().read_line(&mut input2).expect("Failed to load input");
    let papers:i32 = input2.trim().parse().expect("Failed to input");
    
    //count till 100
    let mut count = 0;
    loop{
        count+=1;
        if count ==101{
            break;
        }
    }

    //Conditional statement to determine incentive amount
    if papers >= 10 {
        println!("The incentive for {} is 1,000,000", name);
    }
    else if papers > 5 && papers < 10{
         println!("The incentive for {} is 800,000", name);
    }
    else if papers  >= 3 && papers <= 5 {
            println!("The incentive for {} is 500,000", name);
    }
    else if papers > 3 {
            println!("The incentive for {} is 100,000", name);
        }
    
}