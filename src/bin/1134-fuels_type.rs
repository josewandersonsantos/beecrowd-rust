use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let (mut alcool, mut gas, mut diesel):(i32, i32, i32) = (0, 0, 0);
    
    loop
    {
        let n:i32 = lines.next().unwrap().parse().unwrap();
        if n == 4 {break;}

        match n
        {
            1 => alcool += 1,
            2 => gas += 1,
            3 => diesel += 1,
            _ => {}
        }
    }

    println!("MUITO OBRIGADO");
    println!("Alcool: {}", alcool);
    println!("Gasolina: {}", gas);
    println!("Diesel: {}", diesel);
}