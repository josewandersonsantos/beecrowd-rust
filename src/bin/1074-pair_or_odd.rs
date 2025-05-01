use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let n:u16 = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..n
    {
        let n:i32 = lines.next().unwrap().parse().unwrap();
        
        match n
        {
            0 => println!("NULL"),
            n if n > 0 && n % 2 == 0 => println!("EVEN POSITIVE"),
            n if n < 0 && n % 2 == 0 => println!("EVEN NEGATIVE"),
            n if n > 0 && n % 2 != 0 => println!("ODD POSITIVE"),
            n if n < 0 && n % 2 != 0 => println!("ODD NEGATIVE"),
            _ => {}
        }

        // let num: i32 = lines.next().unwrap().parse().unwrap();
        // if num == 0
        // {
        //     println!("NULL");
        // }
        // else
        // {
        //     let parity = if num % 2 == 0 { "EVEN" } else { "ODD" };
        //     let sign = if num > 0 { "POSITIVE" } else { "NEGATIVE" };
        //     println!("{} {}", parity, sign);
        // }
    }
}