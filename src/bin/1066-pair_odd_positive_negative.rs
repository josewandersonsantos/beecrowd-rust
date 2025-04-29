use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    // let mut pairs=0;
    // let mut odds=0;
    // let mut positives=0;
    // let mut negatives=0;
    // for _ in 0..5
    // {
    //     let n:i32 = lines.next().unwrap().parse().unwrap();
    //     if n % 2 == 0
    //     {
    //         pairs += 1;
    //     }
    //     else
    //     {
    //         odds += 1;    
    //     }

    //     if n > 0
    //     {
    //         positives += 1;
    //     }
    //     else if n < 0
    //     {
    //         negatives += 1;
    //     }
    // }

    let (mut pairs, mut odds, mut positives, mut negatives) = (0, 0, 0, 0);

    for line in lines.take(5)
    {
        let n: i32 = line.parse().expect("Erro ao converter para número");

        match n % 2
        {
            0 => pairs += 1,
            _ => odds += 1,
        }

        match n.cmp(&0)
        {
            std::cmp::Ordering::Greater => positives += 1,
            std::cmp::Ordering::Less => negatives += 1,
            _ => {}
        }
    }

    println!("{} valor(es) par(es)", pairs);
    println!("{} valor(es) impar(es)", odds);
    println!("{} valor(es) positivo(s)", positives);
    println!("{} valor(es) negativo(s)", negatives);
}