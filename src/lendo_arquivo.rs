use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Write;

pub fn lendo_arquivo(){

   // Ler o conteúdo existente do arquivo
    let mut file: File = OpenOptions::new()
        .read(true)
        .append(true)
        .open("dados.txt")
        .expect("Falha ao abrir o arquivo");

    // criando uma variável para receber o conteúdo do arquivo que foi lido..
    let mut content = String::new();

    // lendo o arquivo e colocando o conteúdo dentro da variável content
    file.read_to_string(&mut content)
        .expect("Não conseguir ler o arquivo...");

    // criando uma nova variável com o texto que vai ser inserido..
    let new_content = "Novo conteúdo a ser adicionado.";

    // Abrir o arquivo no modo de escrita para sobreescrever o conteudo antigo..
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("dados.txt")
        .expect("Falha ao abrir o arquivo");

    // Escrever o conteúdo anterior e o texto adicional no arquivo
     file.write_all(content.as_bytes())
        .expect("Falha ao escrever o conteúdo no arquivo");

    file.write_all(new_content.as_bytes())
        .expect("Falha ao escrever o texto adicional no arquivo");

 // Ler o conteúdo atualizado do arquivo
   let mut updated_content = String::new();
    let mut updated_file = OpenOptions::new()
        .read(true)
        .open("dados.txt")
        .expect("Falha ao abrir o arquivo");

    updated_file
        .read_to_string(&mut updated_content)
        .expect("Falha ao ler o conteúdo atualizado do arquivo");
    println!("{}", updated_content)
}
