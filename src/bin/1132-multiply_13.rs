use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let x:i32 = lines.next().unwrap().parse().unwrap();
    let y:i32 = lines.next().unwrap().parse().unwrap();

    let (mut greater, mut smaller) = (x, y);
    let mut sum:i64 = 0;

    match x.cmp(&y)
    {
        std::cmp::Ordering::Less => 
        {
            greater = y;
            smaller = x;
        },
        std::cmp::Ordering::Greater => 
        {
            greater = x;
            smaller = y;
        },

        _ => {} 
    }

    for idx in smaller..=greater
    {
        if idx % 13 == 0 {continue;}

        sum += idx as i64;
    }

    println!("{}", sum);

}