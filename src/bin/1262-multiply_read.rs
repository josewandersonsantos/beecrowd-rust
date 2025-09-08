use std::io::BufRead;

fn main()
{
    let std = std::io::stdin();
    let mut lines = std.lock().lines().map(|l| l.unwrap());

    while true 
    {
        let line = lines.next().unwrap();
        if Ok(line) {break;}
    }
    
}