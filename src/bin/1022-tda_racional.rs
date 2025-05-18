use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let n_cases:u32 = lines.next().unwrap().trim().parse().unwrap();

    for _ in 0..n_cases
    {
        let line = lines.next().unwrap();
        let (mut n_res,mut d_res):(i64, i64) = (0, 0);
        let (n1, _, d1, opr, n2, _, d2) =
        {
            let mut parts = line.split_whitespace();
            (parts.next().unwrap().parse::<i64>().unwrap(), 
             parts.next().unwrap(),
             parts.next().unwrap().parse::<i64>().unwrap(),
             parts.next().unwrap(),
             parts.next().unwrap().parse::<i64>().unwrap(),
             parts.next().unwrap(),
             parts.next().unwrap().parse::<i64>().unwrap())
        };

        match opr
        {
            "+" =>
            {
                n_res = n1*d2 + n2*d1;
                d_res = d1*d2;
            },
            "-" =>
            {
                n_res = n1*d2 - n2*d1;
                d_res = d1*d2;
            },
            "*" =>
            {
                n_res = n1*n2;
                d_res = d1*d2;
            },
            "/" =>
            {
                n_res = n1*d2;
                d_res = d1*n2;
            },
            _ =>{}
        }

        if d_res < 0 && n_res > 0
        {
            n_res *= -1;
            d_res *= -1;
        }
        else if d_res < 0 && n_res < 0
        {
            n_res *= -1;
            d_res *= -1;
        }

        print!("{}/{} = ", n_res, d_res);
        let idx_min = std::cmp::min(n_res.abs(), d_res.abs());
        for idx in (2..=idx_min).rev()
        {
            if n_res % idx == 0 && d_res % idx == 0
            {
                n_res /= idx;
                d_res /= idx;

                if d_res < 0 && n_res > 0
                {
                    n_res *= -1;
                    d_res *= -1;
                }
                else if d_res < 0 && n_res < 0
                {
                    n_res *= -1;
                    d_res *= -1;
                }
                break;
            }
        }
        println!("{}/{}", n_res, d_res);
    }
}