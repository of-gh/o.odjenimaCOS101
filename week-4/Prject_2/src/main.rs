use std::io;

fn main() {
    println!("What is the candidate's full name? ");
    let mut name= String::new();
    io::stdin().read_line(&mut name).expect("Failed to load input");

    println!("If the candidate is experienced, type 'E' and if not type 'I' ");
    let mut input1= String::new();
    io::stdin().read_line(&mut input1).expect("Failed to load input");
    if input1 != ("I") && input1 != ("E") {
        println!("Invalid input. Try again");
    } else if input1 == "I" {
        println!("{}'s incentive is 100,000", name);
    }
    else if input1 == "E" {
        println!("How old is the candidate? ");
        let mut input2= String::new();
        io::stdin().read_line(&mut input2).expect("Failed to load input");
        let age:i32 = input2.trim().parse().expect("Failed to input");

        if age >= 40 {
            println!("{}'s incentive is 1,560,000", name);
        }else if age >= 30 && age < 40{
            println!("{}'s incentive is 1,480,000", name);
        }else if age  <= 28 {
            println!("{}'s incentive is 1,300,000", name);
        }

    }

}
