use std::io::stdin;
fn main()
{
    let mut input:String = String::new();
    let mut total:u32 = 0;

    let grams = [300, 1500, 600, 1000, 150];

    for idx in 0..5
    {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        total = total + input.trim().parse::<u32>().unwrap() * grams[idx] as u32; 
    }

    println!("{}", total + 225);
}