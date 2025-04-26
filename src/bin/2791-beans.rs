use std::io::stdin;
fn main()
{
    let mut input:String = String::new();

    stdin().read_line(&mut input).unwrap();
    let cups:Vec<u8> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    for i  in 0..cups.len()
    {
        if cups[i] == 1
        {
            println!("{}", i+1);
            break;
        }        
    }
}