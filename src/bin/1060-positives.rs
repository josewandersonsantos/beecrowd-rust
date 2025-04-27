use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let mut positives=0;
    for _ in 0..6
    {
        let n:f64 = lines.next().unwrap().parse().unwrap();
        if n > 0.00
        {
            positives += 1;
        }
    }

    println!("{} valores positivos", positives);
}