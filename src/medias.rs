use std::collections::HashMap;

pub enum Estatistica {
    Media(Vec<i32>),
    Mediana(Vec<i32>),
    Moda(Vec<i32>),
}

fn media(numeros: &Vec<i32>) -> f64 {
    let mut soma: i32 = 0;
    let mut media: f64 = 0.0;

    for i in numeros {
        soma += i;
    }

    media = (soma as f64 / numeros.len() as f64) as f64;
    media
}

fn mediana(numeros: &Vec<i32>) -> f64 {
    let numeros_sorted: Vec<i32> = numeros.clone();
    let meio_array = numeros_sorted.len() / 2;
    let eh_par = meio_array % 2 == 0;

    if eh_par {
        return media(&vec![
            numeros_sorted[meio_array],
            numeros_sorted[meio_array - 1],
        ]);
    }

    numeros_sorted[meio_array] as f64
}

fn moda(numeros: &Vec<i32>) -> i32 {
    let mut mapper: HashMap<&i32, _> = HashMap::new();

    for numero in numeros {
        let contar: &mut i32 = mapper.entry(numero).or_insert(0);
        *contar += 1;
    }
;
    let mut maior_valor: i32 = 0;


    for (chave, valor) in &mapper {
        if valor  == mapper.values().max().unwrap(){
            maior_valor = **chave
        }
    }
    maior_valor
}

pub fn calc(op: Estatistica) {
    match op {
        Estatistica::Media(nums) => {
            println!("A méida é calculada para {:?} é => {}", nums, media(&nums))
        }
        Estatistica::Mediana(nums) => println!("A Mediana para {:?} é => {}", nums, mediana(&nums)),
        Estatistica::Moda(nums) => println!("A Moda para {:?} é => {}", nums, moda(&nums)),
    }
}



// Regras de OwnerShip em Rust:
    //-> Cada valor tem um dono;
    //-> Só oide ter um único dono;
    //-> Quando o dono sai do escopo o valor é limpo;
    //-> A posse (Ownership) pode ser movida a outro Dono;
