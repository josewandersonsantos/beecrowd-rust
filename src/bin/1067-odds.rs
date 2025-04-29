use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let n:u32 = lines.next().unwrap().parse().unwrap();

    for i in 1..=n
    {
        if i % 2 != 0
        {
            println!("{}", i);
        }
    }
}