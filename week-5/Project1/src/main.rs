use std::io;
fn main() {
    let p:&str = "Poundo Yam/Edinkaiko Soup";
    let f:&str = "Fried Rice & Chicken";
    let a:&str =  "Amala & Ewedu Soup";
    let e:&str = "Eba & Egusi Soup";
    let w:&str = "White Rice & Stew";
    
    //prices
    let p_price:i32 = 3200;
    let f_price:i32 = 3000;
    let a_price:i32 = 2500;
    let e_price:i32 = 2000;
    let w_price:i32 = 2500;
    
    
    println!("Welcome to the Food Shop.");
    println!("\nThis is what we have today:");
    println!("P = {} - N{}", p, p_price);
    println!("F = {} - N{}", f, f_price);
    println!("A = {} - N{}", a, a_price);
    println!("E = {} - N{}", e, e_price);
    println!("W = {} - N{}", w, w_price);
    
    loop{
        println!("\nWhat would you like?(type the first letter of each item in CAPS. i.e F for Fried rice, or E to Exit):");
        let mut order = String::new();
        io::stdin().read_line(&mut order).expect("Not a valid string");

        if order == "E"{
            break;
        }

        println!("How many of it would you like");
        let qty = String::new();
        io::stdin().read_line(&mut order).expect("Not a valid string");
        let qty:i32 = qty.trim().parse().expect("Please enter a valid number");
        
        let mut item_total = 0;

        if order == "P" {
            println!("{} x {} added to your order.", qty, p);
            item_total = p_price * qty;
        } else if order == "F" {
            println!("{} x {} added to your order.", qty, f);
            item_total = f_price * qty;
        } else if order == "A" {
            println!("{} x {} added to your order.", qty, a);
            item_total = a_price * qty;
        } else if order == "E" {
            println!("{} x {} added to your order.", qty, e);
            item_total = e_price * qty;
        } else if order == "W" {
            println!("{} x {} added to your order.", qty, w);
            item_total = w_price * qty;
        } else {
            println!("Invalid selection, please try again.");
            item_total = 0;
        } 
        
        //calcuating discount
        if item_total > 10_000 {
            let discount = item_total as f32 * 0.05;
            println!("\nYou are eligible for a 5% discount of N{:.2}.", discount);
            item_total -= discount as i32;
        }
    
        println!("\nYour total bill is: N{}", item_total);
        println!("Thank you for shopping with us!");
        
    }
}
