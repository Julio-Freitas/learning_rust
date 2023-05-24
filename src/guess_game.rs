extern crate rand;
use rand::Rng;
use std::io;

pub fn guess_game(){
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
}
