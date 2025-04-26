fn main()
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let mut fatorial: i32 = 1;
    for i in 1..=n
    {
        fatorial *= i;
    }

    println!("{}", fatorial);     
}