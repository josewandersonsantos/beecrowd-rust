use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
  
    loop
    {
        // let (d, n) = {
        //     let mut parts = line.split_whitespace();
        //     (parts.next().unwrap(), parts.next().unwrap())
        // };
        let mut d_n:Vec<String> = lines.next().unwrap().split_whitespace().map(|s| s.to_string()).collect();
        if d_n[0] == "0" && d_n[1] == "0"
        {
            break;
        }

        d_n[1] = d_n[1].to_string().chars().filter(|c| c.to_string() != d_n[0]).collect::<String>();
        if d_n[1].is_empty()
        {
            println!("0");
            continue;
        }

        d_n[1] = d_n[1].trim_start_matches("0").to_string();

        if d_n[1].is_empty()
        {
            println!("0");
        }
        else
        {
            println!("{}", d_n[1]);
        }
    }
}