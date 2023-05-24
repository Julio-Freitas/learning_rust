// mod  lendo_arquivo;
// use lendo_arquivo::lendo_arquivo;

// mod trait_file;
// use trait_file::Pessoa;

// use crate::trait_file::Voz;

// fn count_word(word: &String) -> u32 {
//     let mut contador: u32 = 0;

//     for indice in word.chars() {
//         if indice.is_uppercase() {
//             contador += 1
//         }
//     }
extern crate rand;
use rand::Rng;
use std::io;
//     contador
// }

mod doenca;

mod shapes;
use shapes::Circle;

use crate::shapes::Shape;
use doenca::Doenca;

mod imobiliaria;
use imobiliaria::Imobiliaria;

mod operation;
use operation::{calculate, Operations};

fn main() {
    println!("Bem vindo ao jogo da forca!\n Dica-> frutas");
    let words = vec!["banana", "abacate", "uva", "laranja", "cacau", "caqui"];

    let secret_word = words[rand::thread_rng().gen_range(0..=words.len() - 1)].to_string();

    let mut current_word = vec!['-'; secret_word.len()];
    let mut guess_num: u32 = 0;

    loop {
        println!(
            "Palavra corrente: {}",
            current_word.iter().collect::<String>()
        );
        println!("Adivinhe uma letra:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess = guess.trim().chars().next().unwrap();

        let mut found = false;

        if guess_num == secret_word.chars().count().try_into().unwrap() {
            println!("Tentativa acabaram...");
            break;
        };
        for (i, c) in secret_word.chars().enumerate() {
            if c == guess {
                current_word[i] = c;
                found = true;
            }
        }

        if !found {
            guess_num += 1;
            println!("Letra não encontrada.");
        }

        if current_word.iter().collect::<String>() == secret_word {
            println!("Parabéns, você adivinhou a palavra {}!", secret_word);
            break;
        }
    }

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

}
