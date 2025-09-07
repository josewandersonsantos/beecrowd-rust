use std::{io::BufRead};

fn main()
{
    let std = std::io::stdin();
    let mut lines = std.lock().lines().map(|l| l.unwrap());
    let n_cases = lines.next().unwrap().parse::<usize>().unwrap();

    'outer:for _ in 0..n_cases
    {
        let mut diet = lines.next().unwrap();
        let breakfast = lines.next().unwrap();
        let lunch = lines.next().unwrap();

        let allfood = format!("{}{}", breakfast, lunch);
        
        for food in allfood.chars()
        {
            if !diet.contains(food)
            {
                println!("CHEATER");
                continue 'outer;
            }
        }
        
        for food in allfood.chars()
        {
            if allfood.matches(food).count() > 1
            {
                println!("CHEATER");
                continue 'outer;
            }
        }

        for food in allfood.chars()
        {
            if diet.contains(food)
            {
                diet = diet.replace(food,"");
            }
        }
        let mut chars: Vec<char> = diet.chars().collect();
        chars.sort();
        println!("{}", chars.iter().collect::<String>());
    }
}