use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let mut positives=0;
    let mut avg_positives:f64=0.0;
    for _ in 0..6
    {
        let n:f64 = lines.next().unwrap().parse().unwrap();
        if n > 0.00
        {
            positives += 1;
            avg_positives += n;
        }
    }

    println!("{} valores positivos\n{:.1}", positives, avg_positives / positives as f64);
}