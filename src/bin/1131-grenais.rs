use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let (mut win_inter, mut win_gremio, mut draws, mut games) = (0,0,0,0);
    
    loop
    {
        let numbers:Vec<i32> = lines.next().unwrap().split_whitespace()
            .map(|s| s.parse().unwrap()).collect();
        
        let x:i32 = numbers[0];
        let y:i32 = numbers[1];

        games += 1;
        match x.cmp(&y)
        {
            std::cmp::Ordering::Less => win_gremio += 1,
            std::cmp::Ordering::Greater => win_inter += 1,
            _ => draws += 1,
        }
        
        println!("Novo grenal (1-sim 2-nao)");
        let option:i32 = lines.next().unwrap().parse().unwrap();

        if option != 1
        {
            break;
        }
    }

    println!("{} grenais", games);
    println!("Inter:{}", win_inter);
    println!("Gremio:{}", win_gremio);
    println!("Empates:{}", draws);

    match win_gremio.cmp(&win_inter)
    {
        std::cmp::Ordering::Less => println!("Inter venceu mais"),
        std::cmp::Ordering::Greater => println!("Gremio venceu mais"),
        _ => println!("Nao houve vencedor"),
    }
}