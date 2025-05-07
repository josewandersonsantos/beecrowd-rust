use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let numbers:Vec<i32> = lines.next().unwrap().split_whitespace()
    .map(|s| s.parse().unwrap()).collect();

    for idx in 1..=numbers[1]
    {        
        if idx % numbers[0] == 0
        {
            println!("{}", idx);
        }
        else
        {
            print!("{} ", idx);
        }
    }
}