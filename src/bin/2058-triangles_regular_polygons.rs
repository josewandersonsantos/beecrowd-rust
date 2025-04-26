use std::io::stdin;
fn main()
{
    let mut input:String = String::new();
    stdin().read_line(&mut input).unwrap();

    let vertices:u64 = input.trim().parse().unwrap();

    println!("{}", vertices - 2);
}