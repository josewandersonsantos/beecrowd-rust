use std::io::BufRead;

fn main()
{
    let stdin= std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let n_cases = lines.next().unwrap().parse::<usize>().unwrap();
    let mut cont_food:u32 = 0;
    let mut cont_food_max:u32 = 0;
    let mut revert_line:bool = false;

    for _ in 0..n_cases
    {
        let mut input = lines.next().unwrap();
        input = input.replace(".", "");

        if revert_line
        {
            input = input.chars().rev().collect();
        }
        
        revert_line = !revert_line;

        for c in input.chars()
        {
            if c == 'o'
            {
                cont_food += 1;
            }
            else if c == 'A'
            {
                if cont_food > cont_food_max
                {
                    cont_food_max = cont_food;
                }
                cont_food = 0;
            }
        }
    }
    
    if cont_food > cont_food_max
    {
        cont_food_max = cont_food;
    }
    println!("{}", cont_food_max);
}