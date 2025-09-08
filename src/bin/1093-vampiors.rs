use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());
    loop
    {
        let (mut ev1, mut ev2, at, d) = 
        {
            let input = lines.next().unwrap();
            let mut parts = input.split_whitespace();
            (parts.next().unwrap().parse::<i32>().unwrap(), 
             parts.next().unwrap().parse::<i32>().unwrap(),
             parts.next().unwrap().parse::<i32>().unwrap(),
             parts.next().unwrap().parse::<i32>().unwrap())
        };

        if ev1 == 0 && ev2 == 0 && at == 0 && d == 0 {break;}

        
    }
}