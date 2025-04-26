use std::io::stdin;
fn main()
{
    let mut input:String = String::new();

    stdin().read_line(&mut input).unwrap();
    let infos:Vec<u32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let p1 = infos[0] * infos[1];
    let p2 = infos[2] * infos[3];

    if p1 >  p2
    {
        println!("-1")
    }
    else if p1 < p2
    {
        println!("1")
    }
    else
    {
        println!("0")
    }
}