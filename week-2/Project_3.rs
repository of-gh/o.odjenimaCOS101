fn main(){
    let p:f64 =210000.0;
    let r:f64 =5.0;
    let t:f64 =3.0;

    //depreciation calculation
    let a =  p*(1.0-(r/100.0)).powf (t);
    
    // printing the values
    println!("Value after depreciation is {}", a);
}