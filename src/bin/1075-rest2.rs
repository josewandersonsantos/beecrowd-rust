use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let n:u16 = lines.next().unwrap().parse().unwrap();

    // for idx in 1..10000
    // {
    //     if idx % n == 2
    //     {
    //         println!("{}", idx);
    //     }
    // }

    let mut i = 2;
    while i < 10000
    {
        println!("{}", i);
        i += n;
    }
}