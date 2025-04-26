use std::io::stdin;
fn main()
{
    let mut input:String = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse().unwrap();

    for idx in 1..=n
    {
        println!("{} {} {}", idx, idx*idx, idx*idx*idx);
    }
}