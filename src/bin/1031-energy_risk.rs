use std::io::BufRead;

#[derive(Clone)]
struct Region
{
    id:u32,
    on_off:bool
}

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let n_regions:u32 = lines.next().unwrap().trim().parse().unwrap();
    let mut n_jumps:u32 = 0;
    let mut regions:Vec<Region> = Vec::new();

    for idx in 1..=n_regions
    {
        let region:Region = Region { id: idx, on_off: true };
        regions.push(region);
    }

    loop
    {
        n_jumps += 1;
        if n_jumps >= n_regions { println!("Error"); break;}
        let mut vec_test = regions.to_vec();
        let mut cont_off = 0;
        let mut pos = 0;

        loop
        {
            if vec_test[pos as usize].on_off
            {
                vec_test[pos as usize].on_off = false;
                pos += n_jumps;
                cont_off += 1;
            }
            else
            {    
                pos += 1;
            }

            if pos > vec_test.len() as u32 - 1 
            {
                pos -= vec_test.len() as u32;
            }
            if cont_off == n_regions - 1 
            {
                break;
            }            
        }
        
        let mut last_region:Region = Region { id: 0, on_off: false };

        for rg in vec_test 
        {
            if rg.on_off
            {
                last_region = rg;
                break;
            }
        }

        println!("Last:{} to Jmp:{}", last_region.id, n_jumps);
        if last_region.id == 13 {break;}
    }

    println!("{}", n_jumps);
}