fn main()
{
    let mut j:u8 = 7;
    let mut i:u8 = 1;
    loop
    {
        println!("I={} J={}", i, j);
        if i == 9 && j == 5
        {
            break;
        }

        match j 
        {
            5 =>
            {
                i += 2;
                j = 7;
            }
            _ =>
            {
                j -= 1;
            }
        }
    }
}