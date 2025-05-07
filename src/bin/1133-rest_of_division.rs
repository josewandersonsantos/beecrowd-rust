use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let x:i32 = lines.next().unwrap().parse().unwrap();
    let y:i32 = lines.next().unwrap().parse().unwrap();

    let (mut greater, mut smaller) = (x, y);

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

    for idx in smaller + 1..greater
    {
        let rest:i32 = idx % 5;
        if rest == 2 || rest == 3
        {
            println!("{}", idx);
        }
    }
}