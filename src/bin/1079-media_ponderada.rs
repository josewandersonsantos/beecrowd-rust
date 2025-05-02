use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let n:u32 = lines.next().unwrap().parse().unwrap();

    for _ in 0..n
    {
        let v: Vec<f32> = lines.next().unwrap().split_whitespace()
            .map(|s| s.parse::<f32>().unwrap()).collect();
        let media = (v[0] * 2.0 + v[1] * 3.0 + v[2] * 5.0) / 10.0;
        println!("{:.1}", media);
    }
}