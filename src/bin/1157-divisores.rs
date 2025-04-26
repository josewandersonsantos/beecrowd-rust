use std::io::stdin;
fn main()
{
    let mut input:String = String::new();
    stdin().read_line(&mut input).unwrap();
    let dividendo:u32 = input.trim().parse().unwrap();

    for i in 1..=dividendo
    {
        if dividendo % i == 0
        {
            println!("{}", i);
        }
    }
}