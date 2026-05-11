fn main()
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input = input.trim().split_whitespace();
    let w: i32 = input.next().unwrap().parse().unwrap();
    let h: i32 = input.next().unwrap().parse().unwrap();

    let mut map: Vec<Vec<u8>> = Vec::new();

    for i in 0..h
    {
        map.push(Vec::new());
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut input = input.trim().split_whitespace();
        
        for _ in 0..w
        {
            let c: u8 = input.next().unwrap().parse().unwrap();
            map[i as usize].push(c);
        }
    }

    /*
     * // !TODO UNFINISHED
     */
}