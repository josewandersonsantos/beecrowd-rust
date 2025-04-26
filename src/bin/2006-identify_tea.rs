use std::io::stdin;

fn main()
{
    let mut input:String = String::new();
    
    stdin().read_line(&mut input).expect("");
    let tea:u8 = input.trim().parse().expect("");
    
    input.clear();
    stdin().read_line(&mut input).expect("");

    let choices:Vec<u8> = input.split_whitespace().map(|x| x.parse().expect("")).collect();
    let count_choices_corrects = choices.iter().filter(|&&x| x == tea).count();

    println!("{}", count_choices_corrects);
}