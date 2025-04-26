use std::io::stdin;

fn main()
{
    let mut input:String = String::new();
    stdin().read_line(&mut input).expect("");

    let tomadas:Vec<i32> = input.split_whitespace().map(|x| x.parse().expect("")).collect();

    let total:i32 = tomadas.iter().sum();
    println!("{}", total - 3);
}