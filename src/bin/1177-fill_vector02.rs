fn main()
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n:u32 = input.trim().parse().unwrap();

    let mut cont = 0; 

    for i in 0..1000
    {
        println!("N[{}] = {}", i, cont);
        cont = if cont + 1 >= n { 0 } else { cont + 1 };
    }
}