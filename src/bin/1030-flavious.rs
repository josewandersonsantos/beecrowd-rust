use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let cases:u32 = lines.next().unwrap().trim().parse().unwrap();
    
    for case in 0..cases
    {
        let n_k:Vec<i64> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let mut circle:Vec<i64> = Vec::new();
        for idx in 1..=n_k[0]
        {
            circle.push(idx);
        }

        let mut pos:i64 = circle.len() as i64;

        loop
        {
            pos += n_k[1] - 1;

            while pos as i64 >= circle.len() as i64
            {
                pos -= circle.len() as i64;
            }

            circle.remove(pos as usize);

            if circle.len() == 1
            {
                println!("Case {}: {}", case + 1, circle[0]);
                break;
            }
        }
    }
}