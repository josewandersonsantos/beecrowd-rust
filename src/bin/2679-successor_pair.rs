use std::io::stdin;

fn main()
{
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u32>().unwrap();

    println!("{}", if n%2 == 0{ n + 2} else {n+1});
}