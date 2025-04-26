use std::io::stdin;

fn read_number() -> f64
{
    let mut input:String = String::new();
    stdin().read_line(&mut input).expect("");
    input.trim().parse().expect("")
}

fn main()
{
    let a:f64 = read_number();
    let b:f64 = read_number();
    let c:f64 = read_number();

    let avg:f64 = (a*2. + b*3. + c*5.) / (2.+3.+5.);

    println!("MEDIA = {:.1}", avg); 
}