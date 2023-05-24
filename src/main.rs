extern crate rand;

mod doenca;

mod shapes;
use shapes::Circle;

use crate::shapes::Shape;
use doenca::Doenca;

mod imobiliaria;
use imobiliaria::Imobiliaria;

mod operation;
use operation::{calculate, Operations};
mod order_array;
use order_array::sort_array;
fn main() {
    let c = Circle { r: 5.0 };
    c.area();
    c.perimeter();

    let gripe: Doenca = Doenca::new(
        String::from("Gripe"),
        vec![String::from("Febre")],
        String::from("Baixa de vitaminca c"),
        String::from("Beber bastante água"),
    );

    println!("Doença, {}", gripe);

    let endereco_imobiliaria = imobiliaria::Endereco {
        rua: "Dolomita".to_string(),
        cep: 34600280,
        cidade: "Contagem".to_string(),
        bairro: "Carajas".to_string(),
    };

    let endereco_movel = imobiliaria::Endereco {
        rua: "Santa Maria".to_string(),
        cep: 150350,
        cidade: "Contagem".to_string(),
        bairro: "Pedra Azul".to_string(),
    };
    let mut imobiliaria = Imobiliaria {
        imoveis: Vec::new(),
        endereco: endereco_imobiliaria,
    };

    imobiliaria.novo_imovel(endereco_movel, 2, 4, 55.0, 200.55);

    imobiliaria.listar_imoveis();

    let soma: Operations = Operations::Division(3, -5);
    let result: Result<i32, &str> = calculate(soma);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    let mut array: [i32; 7] = [10, 23, 4, 5, 66, 7, -3];
    sort_array(&mut array);
    println!("{:?}", array)
}
