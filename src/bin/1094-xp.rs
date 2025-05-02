use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let n:u32 = lines.next().unwrap().parse().unwrap();

    let mut c:u32 = 0;
    let mut r:u32 = 0;
    let mut s:u32 = 0;
    let mut total:u32 = 0;

    for _ in 0..n
    {
        let v: Vec<String> = lines.next().unwrap().split_whitespace().map(|s| s.to_string()).collect();
        let qtd:u32 = v[0].parse().unwrap();

        match v[1].as_str()
        {
            "C" => c += qtd,
            "R" => r += qtd,
            "S" => s += qtd,
            _ => {}
        }

        total += qtd;
    }

    println!("Total: {} cobaias", total);
    println!("Total de coelhos: {}", c);
    println!("Total de ratos: {}", r);
    println!("Total de sapos: {}", s);
    println!("Percentual de coelhos: {:.2} %", (c as f32/ total as f32) * 100.0);
    println!("Percentual de ratos: {:.2} %", (r as f32/ total as f32) * 100.0);
    println!("Percentual de sapos: {:.2} %", (s as f32/ total as f32) * 100.0);

}