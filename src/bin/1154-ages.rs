use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let (mut sum, mut cont):(f64, f64) = (0.0,0.0);
    loop
    {
        let x:f64 = lines.next().unwrap().parse().unwrap();
        if x < 0.0 {break;}

        sum += x;
        cont += 1.0;
    }

    println!("{:.2}", (sum /cont))
}