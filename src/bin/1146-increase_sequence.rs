use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    
    loop
    {
        let x:u32 = lines.next().unwrap().parse().unwrap();
        if x == 0 {break;}

        for idx in 1..x
        {
            print!("{} ", idx);
        }
        println!("{}", x);
    }
}