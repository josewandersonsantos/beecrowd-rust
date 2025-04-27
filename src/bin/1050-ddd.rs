use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let ddd:u32 = lines.next().unwrap().parse().unwrap();

    match ddd 
    {
        61 => println!("Brasilia"),
        71 => println!("Salvador"),
        11 => println!("Sao Paulo"),
        21 => println!("Rio de Janeiro"),
        32 => println!("Juiz de Fora"),
        19 => println!("Campinas"),
        27 => println!("Vitoria"),
        31 => println!("Belo Horizonte"),
        _ => println!("DDD nao cadastrado")
    }
}