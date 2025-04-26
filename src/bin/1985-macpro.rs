use std::io::stdin;
struct Product
{
    id:u16,
    price:f32
}

fn main()
{
    let products: [Product; 5] = 
    [
        Product{id: 1001, price: 1.50},
        Product{id: 1002, price: 2.50},
        Product{id: 1003, price: 3.50},
        Product{id: 1004, price: 4.50},
        Product{id: 1005, price: 5.50}
    ];

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let test:u8 = input.trim().parse().unwrap();

    let mut total:f32 = 0.0;

    for _ in 0..test
    {
        input.clear();
        stdin().read_line(&mut input).unwrap();

        let shop:Vec<u16> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

        let product = products.iter().find(|&p| p.id == shop[0]).unwrap();

        total = total + product.price * shop[1] as f32;
    }

    println!("{:.2}", total);
}