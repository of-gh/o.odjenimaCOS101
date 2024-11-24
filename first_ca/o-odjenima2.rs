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