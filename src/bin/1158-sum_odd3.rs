use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let cases:u32 = lines.next().unwrap().trim().parse().unwrap();

    for _ in 0..cases
    {
        let mut x_y:Vec<i64> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let mut sum = 0;
        let mut cont_odd = 0;

        while cont_odd < x_y[1]
        {
            if x_y[0] % 2 != 0
            {
                sum += x_y[0];
                cont_odd += 1;
                //println!("odd {}, sum {}", x_y[0], sum); 
            }
            x_y[0] += 1;
        }
        println!("{}", sum); 
    }
}