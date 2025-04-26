use std::io::{stdin, stdout, Write};
fn main()
{
    let a:i32;
    let b:i32;
    let product:i32;

    let mut input:String = String::new();

    let _ = stdout().flush();
    let _ = stdin().read_line(&mut input);

    a = input.trim().parse().expect("");

    input.clear();

    let _ = stdout().flush();
    let _ = stdin().read_line(&mut input);

    b = input.trim().parse().expect("");

    product = a * b;

    println!("PROD = {}", product);
}