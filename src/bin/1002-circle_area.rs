use std::io::{stdin, stdout, Write};

fn main()
{
    const PI:f64 = 3.14159;

    let r:f64;
    let mut input:String = String::new();

    let _ = stdout().flush(); 
    let _ = stdin().read_line(&mut input);

    r = input.trim().parse().expect("");

    let area:f64 = PI * (r*r); 
    println!("A={:.4}", area);
}