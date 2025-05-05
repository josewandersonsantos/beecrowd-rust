use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let n_cases:u32 = lines.next().unwrap().parse().unwrap();

    for _ in 0..n_cases
    {    
        let numbers:Vec<i32> = lines.next().unwrap().split_whitespace()
            .map(|s| s.parse().unwrap()).collect();
        
        let x:i32 = numbers[0];
        let y:i32 = numbers[1];

        if y == 0
        {
            println!("divisao impossivel");
        }
        else
        {
            let result:f32 = x as f32 / y as f32;
            println!("{:.1}", result);
        }
    }
}