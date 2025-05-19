use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());
    let n_cases = lines.next().unwrap().parse::<usize>().unwrap();

    let (mut pairs, mut odds):(Vec<u32>, Vec<u32>) = (Vec::new(), Vec::new());

    for _ in 0..n_cases
    {
        let n:u32 = lines.next().unwrap().parse().unwrap();

        match n % 2 == 0
        {
            true =>
            {
                // if pairs.len() == 0
                // {
                //     pairs.push(n);
                //     continue;
                // }
                // for idx in 0..=pairs.len()
                // {
                //     if idx == pairs.len()
                //     {
                //         pairs.push(n);
                //         break;
                //     }
                //     if pairs[idx] > n
                //     {
                //         pairs.insert(idx, n);
                //         break;
                //     }
                // }
                match pairs.binary_search_by(|x| x.cmp(&n))
                {
                    Ok(pos) | Err(pos) => pairs.insert(pos, n),
                }
            },
            false =>
            {
                // if odds.len() == 0
                // {
                //     odds.push(n);
                //     continue;
                // }
                // for idx in 0..=odds.len()
                // {
                //     if idx == odds.len()
                //     {
                //         odds.push(n);
                //         break;
                //     }
                //     if odds[idx] < n
                //     {
                //         odds.insert(idx, n);
                //         break;
                //     }
                // }
                match odds.binary_search_by(|x| x.cmp(&n).reverse())
                {
                    Ok(pos) | Err(pos) => odds.insert(pos, n),
                }
            }
        }
    }

    for n in pairs
    {
        println!("{}", n);
    }
    for n in odds
    {
        println!("{}", n);
    }
}