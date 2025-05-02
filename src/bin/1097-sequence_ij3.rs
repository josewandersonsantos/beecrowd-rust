fn main()
{
    let mut j:u8 = 7;
    let mut i:u8 = 1;
    let mut cont:u8 = 2;
    loop
    {
        println!("I={} J={}", i, j);
        if i == 9 && j == 13
        {
            break;
        }

        match cont 
        {
            0 =>
            {
                i += 2;
                j += 4;
                cont = 2;
            }
            _ =>
            {
                cont -= 1;
                j -= 1;
            }
        }
    }
}