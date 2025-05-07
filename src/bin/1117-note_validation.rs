use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let mut notas:Vec<f64> = Vec::new();
    loop
    {
        let n:f64 = lines.next().unwrap().parse().unwrap();

        if n < 0.0 || n > 10.0  
        {
            println!("nota invalida");
            continue;
        }

        notas.push(n);

        if notas.len() == 2
        {
            println!("media = {:.2}", (notas[0] + notas[1]) / 2.0 as f64);
            break;
        }
    }
}