use std::io::stdin;

fn read_number() -> i32
{
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main()
{
    let qtd_sequency:i32 = read_number();
    let mut last_number:i32 = -1;
    let mut qtd_circles:i32 = 0;

    for _ in 0..qtd_sequency
    {
        let number:i32 = read_number();
        if number != last_number
        {
            last_number = number;
            qtd_circles += 1;
        }
    }

    println!("{}", qtd_circles);
}