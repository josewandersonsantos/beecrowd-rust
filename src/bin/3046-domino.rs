use std::io::stdin;
fn main()
{
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u32>().unwrap() + 1;

    println!("{}", n*(n+1)/2);
}