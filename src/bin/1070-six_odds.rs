use std::io::BufRead;
fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let mut n:u32 = lines.next().unwrap().parse().unwrap();
    let mut cont_odds:u8=0;

    loop 
    {
        if cont_odds == 6 {break};

        if n % 2 != 0 
        {
            println!("{}", n);
            cont_odds += 1;
        }
        n +=1;
    }
}