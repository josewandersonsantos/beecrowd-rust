use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let mut pairs=0;
    for _ in 0..5
    {
        let n:i32 = lines.next().unwrap().parse().unwrap();
        if n % 2 == 0
        {
            pairs += 1;
        }
    }

    println!("{} valores pares", pairs);
}