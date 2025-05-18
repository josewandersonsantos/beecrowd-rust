use std::io::BufRead;

fn main()
{
    
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let mut first_case = true;

    loop
    {
        let n_cases:u32 = lines.next().unwrap().trim().parse().unwrap();

        if n_cases == 0
        {
            break;
        }        
        
        let mut strings:Vec<String> = Vec::new();
        let mut string_bigger_size = 0; 
        for _ in 0..n_cases
        {
            let aux_string = lines.next().unwrap().split_whitespace().collect::<Vec<&str>>().join(" ");
            string_bigger_size = std::cmp::max(string_bigger_size, aux_string.len());
            strings.push(aux_string);
        }

        if !first_case
        {
            println!();
        }
        first_case = false;

        for out in strings
        {
            println!("{:>width$}", out, width = string_bigger_size);
        }
    }
}