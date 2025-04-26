use std::io::stdin;
fn main()
{
    let mut input:String = String::new();
    stdin().read_line(&mut input).unwrap();
    let cases:u32 = input.trim().parse().unwrap();

    for _ in 0..cases
    {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let raios:Vec<u32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

        println!("{}", raios.iter().sum::<u32>());
    }
}