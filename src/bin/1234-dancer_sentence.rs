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
        
        let mut result: String = String::new();
        let chars:Vec<char> = line.chars().collect();
        let mut set_upper = true;
        for idx in 0..line.len()
        {
            let c = chars[idx];
            
            if c.is_ascii_whitespace()
            {
                result.push(c);
                continue;
            }
            
            
            match set_upper
            {
                true =>
                {
                    result.push(c.to_ascii_uppercase());
                    set_upper = false;
                },
                false =>
                {
                    result.push(c.to_ascii_lowercase());
                    set_upper = true;
                }
            }
        }

        println!("{}", result);
    }
}