use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let x:u32 = lines.next().unwrap().parse().unwrap();
    let mut cont_lines:u32 = 0;
    let mut cont:u32 = 0;
    
    loop
    {
        cont += 1;
        if  cont_lines == x {break;}
        match cont % 4
        {
            0 => {println!("PUM"); cont_lines += 1;},
            _ => print!("{} ", cont)
        }
    }
}