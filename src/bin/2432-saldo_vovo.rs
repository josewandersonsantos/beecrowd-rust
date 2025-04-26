use std::io::stdin;

fn main()
{
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let values: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut saldo: i32 = values[1];
    let mut min_saldo: i32 = saldo;

    for _ in 0..values[0]
    {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let mov:i32 = input.trim().parse().unwrap();
        saldo += mov;

        min_saldo = if saldo < min_saldo { saldo } else { min_saldo };
    }
    println!("{}", min_saldo);
}