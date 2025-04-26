use std::io::stdin;
fn main()
{
    let mut input:String = String::new();
    stdin().read_line(&mut input).unwrap();
    let n1:u8 = input.trim().parse().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let avg:u8 = input.trim().parse().unwrap();

    let n2:u8 = 2*avg-n1;

    println!("{}", n2);
}