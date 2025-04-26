use std::io::stdin;
fn main()
{
    let mut input=String::new();
    stdin().read_line(&mut input).unwrap();
    let n:u32=input.trim().parse().unwrap();

    for idx in 1..=n
    {
        println!("{} {} {}", idx, idx*idx, idx*idx*idx);
        println!("{} {} {}", idx, idx*idx + 1, idx*idx*idx + 1);
    }
}