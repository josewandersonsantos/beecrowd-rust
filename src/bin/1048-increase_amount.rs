use std::io::stdin;

fn main()
{
    let mut input:String = String::new();
    stdin().read_line(&mut input).unwrap();
    let currently_amount:f64= input.trim().parse().unwrap();

    if currently_amount > 2000.00
    {
        println!("Novo salario: {:.2}", currently_amount * 1.04);
        println!("Reajuste ganho: {:.2}", currently_amount * 0.04);
        println!("Em percentual: 4 %");
    }
    else if currently_amount > 1200.00
    {
        println!("Novo salario: {:.2}", currently_amount * 1.07);
        println!("Reajuste ganho: {:.2}", currently_amount * 0.07);
        println!("Em percentual: 7 %");
    }
    else if currently_amount > 800.00
    {
        println!("Novo salario: {:.2}", currently_amount * 1.10);
        println!("Reajuste ganho: {:.2}", currently_amount * 0.10);
        println!("Em percentual: 10 %");
    }
    else if currently_amount > 400.00
    {
        println!("Novo salario: {:.2}", currently_amount * 1.12);
        println!("Reajuste ganho: {:.2}", currently_amount * 0.12);
        println!("Em percentual: 12 %");
    }
    else 
    {        
        println!("Novo salario: {:.2}", currently_amount * 1.15);
        println!("Reajuste ganho: {:.2}", currently_amount * 0.15);
        println!("Em percentual: 15 %");
    }
}