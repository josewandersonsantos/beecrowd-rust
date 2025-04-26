use std::io::stdin;

fn main()
{
    let mut input=String::new();
    
    loop
    {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        
        if input == "0 0\n" { break; }
        let values:Vec<u64>=input.split_whitespace().map(|x| x.parse().unwrap()).collect();

        println!("{}", values[0] * values[1]);
    }
}