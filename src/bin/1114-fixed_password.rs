use std::io::BufRead;

fn main()
{
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());
    
    loop
    {
        if lines.next().unwrap().eq("2002")
        {
            println!("Acesso Permitido");
            break;
        }
        else
        {
            println!("Senha Invalida");
        } 
    } 
}