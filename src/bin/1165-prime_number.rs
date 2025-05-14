use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let cases:u32 = lines.next().unwrap().trim().parse().unwrap();
    
    'cases_for: for _ in 0 ..cases
    {
        let n:u32 = lines.next().unwrap().trim().parse().unwrap();

        for idx in 2 ..n
        {
            if n % idx == 0
            {
                println!("{} nao eh primo", n);
                continue 'cases_for;
            }
        }
        println!("{} eh primo", n);        
    }
}