use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let (mut a, mut n, mut sum):(i32, i32, i64) = (0, 0, 0);
    
    let numbers:Vec<i32> = lines.next().unwrap().split_whitespace()
        .map(|s| s.parse().unwrap()).collect();
    
    a = numbers[0];

    for idx in 1..numbers.len()
    {
        if numbers[idx] > 0
        {
            n = numbers[idx];
            break;
        }
    }
    
    for idx in 0..n
    {
        sum = sum + (a + idx) as i64;
    }

    println!("{}", sum);
}