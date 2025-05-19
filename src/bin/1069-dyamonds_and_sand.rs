use std::io::BufRead;

fn main()
{
    let stdin= std::io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());
    let n_cases = lines.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..n_cases
    {
        let mut input = lines.next().unwrap();
        let mut diamonds:u32 = 0;

        input = input.replace(".", "");

        loop
        {
            let count = input.matches("<>").count();
            input = input.replace("<>", "");
            diamonds += count as u32;

            if count == 0
            {
                break;
            }
        }
        println!("{}", diamonds);
    }    
}