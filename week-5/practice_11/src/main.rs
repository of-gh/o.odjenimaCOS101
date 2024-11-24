fn main() {
    let a:i32 = 2;
    let b:132 = 3;

    let mut result:132;

    result = a & b;
    println!("(a & b) => {}", result);

    result = a | b;
    println!("(a | b) => {}", result);

    result = a ^ b;
    println!("(a ^ b) => {}", result);

    result = !b;
    println!("(!b) => {}", result);

    result = a << b;
    println!("(a << b) => {}", result);

    result = a >> b;
    println!("(a >> b) => {}", result);
}
