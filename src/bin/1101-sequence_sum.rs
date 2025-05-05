use std::io::BufRead;

fn main()
{
    loop
    {
        let stdin = std::io::stdin();
        let mut lines = stdin.lock().lines().map(|l| l.unwrap());
        
        let numbers:Vec<i32> = lines.next().unwrap().split_whitespace()
            .map(|s| s.parse().unwrap()).collect();
        
        let n:i32 = numbers[0];
        let m:i32 = numbers[1];

        if n <= 0 || m <= 0
        {
            break;
        }

        let greater = n.max(m);
        let smaller = n.min(m);
        let mut sum = 0;
        for idx in smaller..=greater
        {
            sum += idx;

            print!("{} ", idx);
        }

        println!("Sum={}", sum);
    }
}