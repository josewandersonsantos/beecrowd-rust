use std::io::{stdin};

fn read_number() -> f64
{
    let mut input:String = String::new();
    stdin().read_line(&mut input).expect("");
    input.trim().parse().expect("")
}

fn main()
{
    let a:f64;
    let b:f64;
    let avg:f64;

    a = read_number();
    b = read_number();

    avg = (a * 3.5 + b * 7.5) / (3.5 + 7.5);

    println!("MEDIA = {:.5}", avg);
}