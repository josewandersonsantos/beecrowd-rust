// use std::io::stdin;

// fn main()
// {
//     let mut input = String::new();
//     stdin().read_line(&mut input).unwrap();
//     let first_class = input.trim();
    
//     match first_class
//     {        
//         "vertebrado" => 
//         {
//             input.clear();
//             stdin().read_line(&mut input).unwrap();
//             let second_class = input.trim();

//             match second_class
//             {
//                 "ave" => 
//                 {
//                     input.clear();
//                     stdin().read_line(&mut input).unwrap();
//                     let third_class = input.trim();

//                     match third_class
//                     {
//                         "carnivoro" => 
//                         {
//                             println!("aguia");
//                             return;
//                         }
//                         "onivoro" => 
//                         {
//                             println!("pomba");
//                             return;
//                         }
//                         _ => 
//                         {
//                             println!("Unknown second class");
//                             return;
//                         }    
//                     }
//                 }      
//                 "mamifero" => 
//                 {
//                     input.clear();
//                     stdin().read_line(&mut input).unwrap();
//                     let third_class = input.trim();

//                     match third_class
//                     {
//                         "onivoro" => 
//                         {
//                             println!("homem");
//                             return;
//                         }
//                         "herbivoro" => 
//                         {
//                             println!("vaca");
//                             return;
//                         }
//                         _ => 
//                         {
//                             println!("Unknown second class");
//                             return;
//                         }    
//                     }
//                 }      
//                 _ => 
//                 {
//                     println!("Unknown second class");
//                     return;
//                 }      
//             }
//         }
//         "invertebrado" => 
//         {
//             input.clear();
//             stdin().read_line(&mut input).unwrap();
//             let second_class = input.trim();

//             match second_class
//             {
//                 "inseto" => 
//                 {
//                     input.clear();
//                     stdin().read_line(&mut input).unwrap();
//                     let third_class = input.trim();

//                     match third_class
//                     {
//                         "hematofago" => 
//                         {
//                             println!("pulga");
//                             return;
//                         }
//                         "herbivoro" => 
//                         {
//                             println!("lagarta");
//                             return;
//                         }
//                         _ => 
//                         {
//                             println!("Unknown second class");
//                             return;
//                         }    
//                     }
//                 }      
//                 "anelideo" => 
//                 {
//                     input.clear();
//                     stdin().read_line(&mut input).unwrap();
//                     let third_class = input.trim();

//                     match third_class
//                     {
//                         "hematofago" => 
//                         {
//                             println!("sanguessuga");
//                             return;
//                         }
//                         "onivoro" => 
//                         {
//                             println!("minhoca");
//                             return;
//                         }
//                         _ => 
//                         {
//                             println!("Unknown second class");
//                             return;
//                         }    
//                     }
//                 }      
//                 _ => 
//                 {
//                     println!("Unknown second class");
//                     return;
//                 }      
//             }
//         }
//         _ => 
//         {
//             println!("Unknown first class");
//             return;
//         }
//     }


// }

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let first_class = lines.next().unwrap().trim().to_string();
    let second_class = lines.next().unwrap().trim().to_string();
    let third_class = lines.next().unwrap().trim().to_string();

    let animal = match (first_class.as_str(), second_class.as_str(), third_class.as_str()) {
        ("vertebrado", "ave", "carnivoro") => "aguia",
        ("vertebrado", "ave", "onivoro") => "pomba",
        ("vertebrado", "mamifero", "onivoro") => "homem",
        ("vertebrado", "mamifero", "herbivoro") => "vaca",
        ("invertebrado", "inseto", "hematofago") => "pulga",
        ("invertebrado", "inseto", "herbivoro") => "lagarta",
        ("invertebrado", "anelideo", "hematofago") => "sanguessuga",
        ("invertebrado", "anelideo", "onivoro") => "minhoca",
        _ => "Desconhecido",
    };

    println!("{}", animal);
}
