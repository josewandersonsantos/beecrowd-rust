use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let (mut greater, mut index) = (0,0);
    
    for idx in 1..=100
    {
        let n:u32 = lines.next().unwrap().parse().unwrap();

        if n > greater
        {
            greater = n;
            index = idx;
        }
    }

    println!("{}\n{}", greater, index);
}
