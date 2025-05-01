use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let n:u16 = lines.next().unwrap().parse().unwrap();

    for idx in 1..=10
    {
        println!("{} x {} = {}", idx, n, n * idx);
    }
}