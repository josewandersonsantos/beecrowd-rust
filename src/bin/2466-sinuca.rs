use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let _n_balls = lines.next().unwrap();

    let mut balls: Vec<i32> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
    
    // for n in 0..balls.len()
    // {
    //     print!("{} ", balls[n]);
    // }
    // println!();

    while balls.len() > 1
    {
        let mut nex_line: Vec<i32> = Vec::new();

        for idx in 0..balls.len() - 1
        {
            let sum_balls = balls[idx] + balls[idx + 1];
            if sum_balls == 0
            {
                nex_line.push(-1);
            }
            else
            {
                nex_line.push(1);
            }
        }

        balls = nex_line;

        // for n in 0..balls.len()
        // {
        //     print!("{} ", balls[n]);
        // }        
        // println!();
    }

    if balls.len() == 1 && balls[0] == -1
    {
        println!("branca");
    }
    else
    {
        println!("preta");
    }
}