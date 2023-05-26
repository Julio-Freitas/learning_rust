use std::collections::HashMap; // collection Hashmap stores a mapping of keys of type k to values of type V
use std::io; // it is module default rust to give user input and output

struct User {
    name: String,
    age: u32,
    friends: Vec<String>,
}

pub fn social_media() {
    let mut users: HashMap<String, User> = HashMap::new();

    loop {
        println!("Escolha uma opção:\n1.Adicionar usuário\n2.Adicionar amigo\n3.Ver lista de amigos\n4.Sair\n");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap();

        match choice {
            1 => {
                println!("Digite o nome:");
                let mut name: String = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim().to_string();

                println!("Digite a idade:");
                let mut age: String = String::new();
                io::stdin().read_line(&mut age).unwrap();
                let age: u32 = age.trim().parse().unwrap();

                let user: User = User {
                    name: name.clone(),
                    age,
                    friends: Vec::new(),
                };
                users.insert(name, user);
            }
            2 => {
                println!("Digite o nome do usuário:");
                let mut user_name = String::new();
                io::stdin().read_line(&mut user_name).unwrap();
                let mut user_name = user_name.trim().to_string();

                println!("Digite o nome do amigo:");
                let mut friend_name = String::new();
                io::stdin().read_line(&mut friend_name).unwrap();
                let friend_name = friend_name.trim().to_string();

                users
                    .get_mut(&mut user_name)
                    .unwrap()
                    .friends
                    .push(friend_name);
            }
            3 => {
                println!("Digite o nome do usuário:");
                let mut user_name = String::new();
                io::stdin().read_line(&mut user_name).unwrap();
                let name = user_name.trim().to_string();
                let list = users.get(&name).unwrap();

                println!("{}, Sua lista de amigos: {:?}", user_name, list.friends);
            }
            4 => break,
            _ => println!("Opçção não é valida"),
        }
    }
}
