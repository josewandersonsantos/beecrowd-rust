use std::io::stdin;

fn main()
{
    let mut input:String = String::new();

    stdin().read_line(&mut input).expect("");
    let clicks:i32 = input.trim().parse().expect("");

    // c2 = 2 * c3
    // c2 = c1 / 2
    // c1/2 = 2*c3
    // c1 = 4*c3

    println!("{}", clicks * 4);
}