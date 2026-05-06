fn main()
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u32>().unwrap();

    for _ in 0..n-1
    {
        print!("Ho ");
    }

    println!("Ho!");
}