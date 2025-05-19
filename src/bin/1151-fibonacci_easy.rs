use std::io::BufRead;

static mut CACHE: [i32; 1000] = [-1; 1000];

fn fibonacci(n:i32) -> i32
{
    if n == 0 
    {
        //println!("0");
        return 0;
    }
    
    if n == 1 
    {
        //println!("1");
        return 1;
    }

    unsafe 
    {
        if CACHE[n as usize] != -1
        {
            return CACHE[n as usize];
        }
        CACHE[n as usize] = fibonacci(n - 1) + fibonacci(n - 2);
        CACHE[n as usize]
    }  
}

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let n: i32 = lines.next().unwrap().parse().unwrap();

    if n == 0
    {
        return;
    }

    if n == 1
    {
        println!("0");
        return;
    }

    if n == 2
    {
        println!("0 1");
        return;
    }

    for idx in 0..n
    {
        if idx == n-1
        {
            println!("{}", fibonacci(idx));
        }
        else
        {
            print!("{} ", fibonacci(idx));
        }
    }
}