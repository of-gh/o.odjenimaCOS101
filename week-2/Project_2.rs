fn main(){
    let t:f64 =450000.0;
    let m:f64 =1500000.0;
    let h:f64 =750000.0;
    let d:f64 =2850000.0;
    let a:f64 =250000.0;

    //calculate sum and average
    let s = t+m+h+d+a;
    let ave = s/5.0;

    // printing the results
    println!("The sum is {}", s);
    println!("The average is {}", ave);
}