use std::io::{stdin, stdout, Write};

//#[derive(Debug)]
struct Player
{
    id: u8,
    //valid: bool,
}

fn get_order_players(players:Vec<i32>) -> Vec<Player>
{
    let mut line_players: Vec<Player> = Vec::new();

    for i in 0..players.len()
    {
        let player: Player = Player
        {
            id: players[i] as u8,
            //valid: true,
        };
        line_players.push(player);
    }

    line_players
}

fn main()
{    
    let mut count_tests:i32 = 0;
    loop
    {
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if input == "0 0\n"
        {
            break;
        }

        count_tests += 1;

        // Get total players and games
        let players_and_games: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        
        input.clear();
        stdin().read_line(&mut input).unwrap();
        
        // Get total players and games
        let line_players: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        // Get Line players
        let mut line_players: Vec<Player> = get_order_players(line_players.clone());
        
        //println!("line_players: {:?}", line_players);
        // Exec rounds
        for _ in 0..players_and_games[1]
        {
            input.clear();
            stdin().read_line(&mut input).unwrap();

            let info:Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
            let cmd = info[1];

            // println!("info: {:?}", info);
            // println!("cmd: {}", cmd);
            
            // Get players choices
            let choices_players:Vec<i32> = (&info[2..]).to_vec();
            //println!("choice_players : {:?}", choices_players);

            //line_players = line_players.into_iter().filter(|x| x.valid).collect();
            line_players = line_players.into_iter()
            .enumerate()
            .filter(|(i, _)| choices_players[*i] == cmd)
            .map(|(_, p)| p)
            .collect();
            
            if line_players.len() == 1
            {
                println!("Teste {}\n{}\n", count_tests, line_players[0].id);
                break;
            }
        }
    }
}

// use std::io::{self, BufRead};

// fn main() {
//     let stdin = io::stdin();
//     let mut lines = stdin.lock().lines();

//     let mut teste_num = 1;

//     while let Some(Ok(line)) = lines.next() {
//         let parts: Vec<usize> = line
//             .split_whitespace()
//             .map(|x| x.parse().unwrap())
//             .collect();

//         let (p, r) = (parts[0], parts[1]);

//         if p == 0 && r == 0 {
//             break;
//         }

//         // Lê a fila inicial
//         let fila_inicial: Vec<usize> = lines
//             .next()
//             .unwrap()
//             .unwrap()
//             .split_whitespace()
//             .map(|x| x.parse().unwrap())
//             .collect();

//         let mut fila = fila_inicial;

//         for _ in 0..r {
//             let rodada: Vec<usize> = lines
//                 .next()
//                 .unwrap()
//                 .unwrap()
//                 .split_whitespace()
//                 .map(|x| x.parse().unwrap())
//                 .collect();

//             let n = rodada[0];
//             let ordem = rodada[1];
//             let acoes = &rodada[2..];

//             // Elimina quem errou
//             fila = fila
//                 .into_iter()
//                 .zip(acoes.iter())
//                 .filter(|(_, &acao)| acao == ordem)
//                 .map(|(id, _)| id)
//                 .collect();
//         }

//         println!("Teste {}\n{}\n", teste_num, fila[0]);
//         teste_num += 1;
//     }
// }
