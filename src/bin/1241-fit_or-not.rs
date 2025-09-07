use std::io::BufRead;


fn main()
{
    let std = std::io::stdin();
    let mut lines = std.lock().lines().map(|l| l.unwrap());
    let n_cases = lines.next().unwrap().parse::<usize>().unwrap();
    let (mut _a, mut _b):(String, String) = (String::new(), String::new());
    for _ in 0..n_cases
    {
        let line = lines.next().unwrap();
        let mut parts = line.split_whitespace();
        _a = parts.next().unwrap().to_string();
        _b = parts.next().unwrap().to_string();

        if _b.len() > _a.len()
        {
            println!("nao encaixa");
        }
        // else if &_a[_a.len() - _b.len()..] == _b
        else if _a.ends_with(&_b)
        {
            println!("encaixa");
        }
        else
        {
            println!("nao encaixa");
        }
    }
}