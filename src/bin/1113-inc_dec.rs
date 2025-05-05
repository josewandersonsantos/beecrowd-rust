use std::io::BufRead;

fn main()
{
    loop
    {
        let stdin = std::io::stdin();
        let mut lines = stdin.lock().lines().map(|l| l.unwrap());
        let numbers:Vec<i32> = lines.next().unwrap().split_whitespace()
            .map(|s| s.parse().unwrap()).collect();
        
        let x:i32 = numbers[0];
        let y:i32 = numbers[1];

        if x == y
        {
            break;
        }

        match x.cmp(&y)
        {
            std::cmp::Ordering::Less => println!("Crescente"),
            std::cmp::Ordering::Greater => println!("Decrescente"),
            _ => {}
        }
    }
}