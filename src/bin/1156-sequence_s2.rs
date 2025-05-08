fn main()
{
    let (mut s, mut num, mut den):(f64, f64, f64) = (1.0, 1.0, 1.0);

    loop
    {
        num += 2.0;
        den = den * 2.0;
        s = s + (num/den);

        //println!("+ {:.2}/{:.2}", num, den);

        if num == 39.0 {break;}    
    }

    println!("{:.2}", s);
}