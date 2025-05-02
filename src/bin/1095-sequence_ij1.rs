fn main()
{
    let mut j:u8 = 65;
    let mut i:i8 = -2;
    while j != 0
    {
        j -= 5;
        i += 3;
        println!("I={} J={}", i, j);
    }
}