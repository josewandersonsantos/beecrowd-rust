use std::io::stdin;

fn main()
{
    let mut input: String = String::new();
    stdin().read_line(&mut input).unwrap();
    let c_p_f: Vec<u32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    if (c_p_f[0] * c_p_f[2]) <= c_p_f[1]
    {
        println!("S");
    }
    else
    {
        println!("N");
    }
}