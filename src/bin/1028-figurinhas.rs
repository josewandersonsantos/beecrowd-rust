use std::io::BufRead;

fn euclides(a:i32, b:i32) -> i32
{
    if a == 0
    {
        return b;
    }
    else if b == 0
    {
        return a;
    }

    euclides(b, a % b)
}

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());
    let n_cases: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..n_cases
    {
        let input = lines.next().unwrap();
        let (f1, f2) =
        {
            let mut parts = input.split_whitespace();
            (parts.next().unwrap().parse::<i32>().unwrap(), 
             parts.next().unwrap().parse::<i32>().unwrap())
        };

        let (min, max) = (std::cmp::min(f1, f2), std::cmp::max(f1, f2));

        if max % min == 0
        {
            println!("{}", min);
            continue;
        }

        println!("{}", euclides(min, max));
    }
}