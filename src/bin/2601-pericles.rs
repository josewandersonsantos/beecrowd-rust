use std::io::stdin;

fn main()
{
    let mut input:String = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut tab_action:Vec<u16> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    for _ in 0..tab_action[1]
    {
        let mut input:String = String::new();
        stdin().read_line(&mut input).unwrap();

        if input.trim() == "fechou"
        {
            tab_action[0] = tab_action[0] + 1;
        }
        else
        {
            tab_action[0] = tab_action[0] - 1;
        }
    }

    println!("{}", tab_action[0]);
}