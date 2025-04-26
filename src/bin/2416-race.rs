use std::io::stdin; 

fn main()
{
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n:Vec<u64> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    println!("{}", n[0] % n[1]);
}