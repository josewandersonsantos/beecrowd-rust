use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());
    let n_cases:u32 = lines.next().unwrap().parse().unwrap();

    for _ in 0..n_cases
    {
        let (a, b) =
        {
            let input = lines.next().unwrap();
            let mut parts = input.split_whitespace();
            (parts.next().unwrap().to_string(), 
             parts.next().unwrap().to_string())
        };

        if a.ends_with(b.as_str())
        {
            println!("encaixa");
        }
        else
        {
            println!("nao encaixa");
        }
    }
}