fn main(){
    let p:f64 =520000000.0;
    let r:f64 =10.0;
    let t:f64 =5.0;

    //compound interest
    let a = p*(1.0+(r/100.0))*t;
    let ci = a-p;
    
    // printing the values
    println!("Amount is {}", a);
    println!("Compound interest is {}", ci);
}