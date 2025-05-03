use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let n_cases:u32 = lines.next().unwrap().parse().unwrap();

    for _ in 0..n_cases
    {
        let x_y: Vec<u32> = lines.next().unwrap().split_whitespace()
            .map(|s| s.parse().unwrap()).collect();

        let mut sum = 0;

        let greater = x_y[0].max(x_y[1]);
        let smaller = x_y[0].min(x_y[1]);

        for idx in smaller + 1..greater
        {
            if idx % 2 == 1
            {
                sum += idx;
            }
        }

        println!("{}", sum);
    }
}