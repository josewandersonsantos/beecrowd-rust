use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    
    loop
    {        
        let mut n:i64 = lines.next().unwrap().trim().parse().unwrap();
        if n == 0 {break;}
        
        let mut sum = 0;
        let mut cont_odd = 0;
        
        while cont_odd < 5
        {
            if n % 2 == 0
            {
                sum += n;
                cont_odd += 1;
                //println!("odd {}, sum {}", n, sum); 
            }
            n += 1;
        }
        println!("{}", sum); 
    }
}