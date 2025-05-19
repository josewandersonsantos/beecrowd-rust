use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap());

    for line in lines
    {
        if line.is_empty()
        {
            break;
        }

        let (v, t) =
        {
            let mut parts = line.split_whitespace();
            (parts.next().unwrap().parse::<i32>().unwrap(), 
             parts.next().unwrap().parse::<i32>().unwrap())
        };

        println!("{}", v * t * 2);
    }
}