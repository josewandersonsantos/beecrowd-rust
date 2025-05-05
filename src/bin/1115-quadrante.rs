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

        if x == 0 || y == 0
        {
            break;
        }

        match x > 0 
        {
            true => match y > 0
            {
                true => println!("primeiro"),
                false => println!("quarto"),
            },
            false => match y > 0
            {
                true => println!("segundo"),
                false => println!("terceiro"),
            }
        }

    }
}