use std::io::stdin;
fn main()
{
    let mut input:String = String::new();
    stdin().read_line(&mut input).unwrap();
    let position:u8 = input.trim().parse().unwrap(); 
    if position > 50
    {
        println!("Top 100");
    }
    else if position > 25
    {
        println!("Top 50");
    } 
    else if position > 10
    {
        println!("Top 25");
    }
    else if position > 5
    {
        println!("Top 10");
    }
    else if position > 3
    {
        println!("Top 5");
    }
    else if position > 1
    {
        println!("Top 3");
    }
    else    
    {
        println!("Top 1");
    }
}