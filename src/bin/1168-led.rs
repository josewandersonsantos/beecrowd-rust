use std::io::BufRead;

fn get_n_led_by_char(c:char) -> u64
{
    match c 
    {
        '0' => 6,
        '1' => 2,
        '2' => 5,
        '3' => 5,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 3,
        '8' => 7,
        '9' => 6,
        _   => 0
    }
}

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let n_cases:u32 = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..n_cases
    {
        let number:String = lines.next().unwrap();
        let mut n_leds:u64 = 0;

        for c in number.chars()
        {
            n_leds += get_n_led_by_char(c);
        }

        println!("{} leds", n_leds);        
    }
}