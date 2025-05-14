use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let mut vector:Vec<i32> = Vec::new();
    for _ in 0 ..10
    {
        let n:i32 = lines.next().unwrap().trim().parse().unwrap();
        match n <= 0
        {
            true => vector.push(1),
            false => vector.push(n),
        }
    }

    vector.iter().enumerate().for_each(|(i, &n)| println!("X[{}] = {}", i, n));
}