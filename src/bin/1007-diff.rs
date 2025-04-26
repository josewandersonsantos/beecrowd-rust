use std::io::stdin;

fn read_number() -> i32
{
    let mut input:String = String::new();
    stdin().read_line(&mut input).expect("");
    input.trim().parse().expect("")
}

fn main()
{
    let a:i32 = read_number();
    let b:i32 = read_number();
    let c:i32 = read_number();
    let d:i32 = read_number();

    let diff:i32 = a*b - c*d;

    println!("DIFERENCA = {}", diff); 
}