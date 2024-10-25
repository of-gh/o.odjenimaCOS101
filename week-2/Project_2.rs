fn main(){
    let t:f64 =450000.0;
    let m:f64 =1500000.0;
    let h:f64 =750000.0;
    let d:f64 =2850000.0;
    let a:f64 =250000.0;

    let tqty:f64 =2.0;
    let mqty:f64=1.0;
    let hqty:f64=3.0;
    let dqty:f64=3.0;
    let aqty:f64=1.0;

    //calculate sum and average
    let qty= tqty+mqty+hqty+dqty+aqty;
    let s = (t*tqty)+(m*mqty)+(h*hqty)+(d*dqty)+(a*aqty);
    let ave = s/(qty);

    // printing the results
    println!("The sum is {}", s);
    println!("The average is {}", ave);
}