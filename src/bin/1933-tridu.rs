use std::io::stdin;
fn main()
{
    let mut input:String = String::new();
    stdin().read_line(&mut input).unwrap();

    let cartas:Vec<u8> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    if cartas[0] == cartas[1]
    {
        println!("{}", cartas[0]);
    }
    else if cartas[0] > cartas[1]
    {
        println!("{}", cartas[0]);
    }
    else
    {
        println!("{}", cartas[1]);        
    }
}