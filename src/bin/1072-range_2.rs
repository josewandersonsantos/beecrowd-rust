use std::io::BufRead;
fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let n:i32 = lines.next().unwrap().parse().unwrap();

    let (mut cont_in, mut cont_out) = (0,0);
    for _ in 0..n
    {
        let x:i32 = lines.next().unwrap().parse().unwrap();
        // if x >= 10 && x <= 20
        // {
        //     cont_in += 1;
        // }
        // else
        // {
        //     cont_out += 1;
        // }
        match x
        {
            10..=20 => cont_in += 1,
            _ => cont_out += 1,
        }
    }
    println!("{} in", cont_in);
    println!("{} out", cont_out);
}