use std::io::BufRead;
fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let n:u32 = lines.next().unwrap().parse().unwrap();

    for idx in 1..=n
    {
        match idx % 2
        {
            0 => println!("{}^{} = {}", idx, 2, idx.pow(2)),
            _ => {}
        }
    }
}