use std::{io::BufRead};


fn main()
{
    let std= std::io::stdin();
    let mut lines = std.lock().lines().map(|l| l.unwrap());
    let n_cases = lines.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..n_cases
    {
        let cipher = lines.next().unwrap();
        let shift = lines.next().unwrap().parse::<u8>().unwrap();

        let mut result = String::new();

        for c in cipher.chars().map(|c | c as u8).collect::<Vec<u8>>()
        {
            let mut new_c: u8 = c - shift;
            //println!("C:{} SH:{} N:{} {}", c, shift, new_c as char, new_c as u8);
            while new_c < 65
            {
                new_c += 26;
            }

            result.push((new_c) as char);
        }

        //println!("{} {} {}",cipher, shift, result.to_ascii_uppercase());
        println!("{}", result.to_ascii_uppercase());
    }
}