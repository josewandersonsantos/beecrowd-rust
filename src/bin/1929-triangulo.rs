fn main()
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let (a, b, c, d) =
    {
        let mut parts = input.split_whitespace();
        (parts.next().unwrap().parse::<i32>().unwrap(), 
        parts.next().unwrap().parse::<i32>().unwrap(),
        parts.next().unwrap().parse::<i32>().unwrap(),
        parts.next().unwrap().parse::<i32>().unwrap())
    }; 
    
    if a + b > c && a + c > b && b + c > a
    {
        println!("S");
    }
    else if a + b > d && a + d > b && b + d > a
    {
        println!("S");
    }
    else if a + c > d && a + d > c && c + d > a
    {
        println!("S");
    }
    else if b + c > d && b + d > c && c + d > b
    {
        println!("S");
    }
    else
    {
        println!("N");
    }
}
