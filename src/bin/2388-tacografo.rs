fn main()
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let leituras: i32 = input.trim().parse().unwrap();
    let mut total: i32 = 0;

    for _ in 0..leituras
    {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();

        let line:Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        total = line[0] * line[1] + total;
    }
    println!("{}", total);
}