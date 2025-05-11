use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let cases:u32 = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..cases
    {
        let n:u64 = lines.next().unwrap().trim().parse().unwrap();
        let mut sum:u64 = 0;

        for i in 1..n
        {
            if n % i == 0
            {
                sum += i;
            }            
        }

        println!("{} {}", n, if sum == n { "eh perfeito" } else { "nao eh perfeito" } );
    }    
}