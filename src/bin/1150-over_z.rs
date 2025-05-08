use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let mut x:u64 = lines.next().unwrap().parse().unwrap();
    let mut z:u64 = 0;
    loop
    {
        z = lines.next().unwrap().parse().unwrap();
        if z > x {break;}
    }

    let (mut sum, mut cont):(u64, u32)  = (x, 1);
    
    loop
    {
        x += 1;
        sum += x;
        cont+= 1;

        if sum > z {break;}
    }

    println!("{}", cont);

}