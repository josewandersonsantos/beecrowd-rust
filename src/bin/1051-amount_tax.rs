use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let amount: f64 = lines.next().unwrap().parse().unwrap();

    if amount > 4500.00
    {
        println!("R$ {:.2}", (amount - 4500.00) * 0.28 + 1500.00 * 0.18 + 1000.00 * 0.08);
    }
    else if amount > 3000.00
    {
        println!("R$ {:.2}", (amount - 3000.00) * 0.18 + 1000.00 * 0.08);
    }
    else if amount > 2000.00
    {
        println!("R$ {:.2}", (amount - 2000.00) * 0.08);
    }
    else
    {
        println!("Isento")    
    }
}