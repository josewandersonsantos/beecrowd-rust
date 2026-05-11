
const PI:f64 = 3.1415;

fn getVol(radio:f64) ->f64
{
    (4.0 / 3.0) * PI * radio.powi(3)
}

fn main()
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input = input.trim().split_whitespace();
    let radio: f64 = input.next().unwrap().parse().unwrap();
    let helio: f64 = input.next().unwrap().parse().unwrap();

    println!("{}", (helio / getVol(radio)) as i32);
}