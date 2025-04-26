use std::io::stdin;

fn main()
{
    let mut input:String = String::new();
    stdin().read_line(&mut input).unwrap();

    let nota_number:u8 = input.trim().parse().unwrap();

    let nota_letter =
    if nota_number >=1 && nota_number < 36
    {
        'D'
    }
    else if nota_number >=36 && nota_number < 61
    {
        'C'
    }
    else if nota_number >=61 && nota_number < 86
    {
        'B'
    }
    else if nota_number >=86 && nota_number < 101
    {
        'A'
    }
    else
    {
        'E'
    };

    println!("{}", nota_letter);
}