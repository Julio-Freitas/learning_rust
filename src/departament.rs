use std::{collections::HashMap, io};

pub fn departament() {
    let mut derpartament_people = HashMap::new();

    loop {
        println!("Digite o comando do tipo: Adicione <Pessoa> para <Departamento> ");
        let mut command: String = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Erro ao ler a variável comando...");

        let command = command.trim();

        let (people, departament) = {
            let mut token_command = command.split_whitespace();

            let people = match token_command.nth(1) {
                Some(p) => p,
                None => {
                    println!("O comando digitado não é válido");
                    continue;
                }
            };

            let departament = match token_command.nth(1) {
                Some(d) => d,
                None => {
                    println!("O comando digitado não é válido");
                    continue;
                }
            };
            (people, departament)
        };

        let employee = derpartament_people
            .entry(String::from(departament))
            .or_insert(vec![]);
        employee.push(String::from(people));
        println!("{:?}", derpartament_people);
    }
}
