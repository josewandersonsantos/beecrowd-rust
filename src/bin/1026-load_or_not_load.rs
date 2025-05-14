use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    
    loop
    {
        let ln = lines.next();
        if ln.is_none()
        {
            break;
        }
        else 
        {
            let x_y:Vec<i64> = ln.unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
            if x_y.len() == 0 {break;}
            println!("{}", (!x_y[0] & x_y[1]) | (x_y[0] & !x_y[1]));
        }
    }
}