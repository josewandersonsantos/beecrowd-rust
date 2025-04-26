use std::io::stdin;

fn main()
{
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let pression_desire:i32 = input.trim().parse().unwrap();
    
    input.clear();

    stdin().read_line(&mut input).unwrap();
    let pression_actual:i32 = input.trim().parse().unwrap();

    println!("{}", pression_desire - pression_actual);
}