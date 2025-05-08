fn main()
{
    let mut s:f64 = 1.0;

    for idx in 2..=100
    {
        s = s + (1.0/idx as f64);
    }

    println!("{:.2}", s);
}