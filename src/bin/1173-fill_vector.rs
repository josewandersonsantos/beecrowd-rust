use std::io::stdin;

fn main()
{
    let mut input:String = String::new();
    stdin().read_line(&mut input).unwrap();
    let n:u32 = input.trim().parse().unwrap();
    let mut vector:Vec<i32> = Vec::new();

    vector.push(n as i32);
    println!("N[{0}] = {1}", 0, vector[0]);
    for i in 1..10
    {
        vector.push(vector[i-1] * 2);
        println!("N[{0}] = {1}", i, vector[i]);
    }
}