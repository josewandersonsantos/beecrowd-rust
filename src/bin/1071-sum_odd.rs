use std::io::BufRead;
fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let n1:i32 = lines.next().unwrap().parse().unwrap();
    let n2:i32 = lines.next().unwrap().parse().unwrap();
    let (greater, lower) = (n1.max(n2), n1.min(n2));

    // for i in lower+1..greater
    // {
    //     if i % 2 != 0
    //     {
    //         sum_odds += i;
    //     }
    // }

    let sum_odds: i32 = (lower + 1..greater).filter(|x| x % 2 != 0).sum();

    println!("{}", sum_odds);
}