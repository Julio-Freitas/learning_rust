pub struct Pessoa {
   pub nome: String,
   pub idade:i32
}

pub trait Voz {
    fn falar(&self);
    fn tem_voz(&self) -> bool;
}


impl Voz for Pessoa {
    fn falar(&self){
        println!("Ola, meu nome eh {}", self.nome);
    }

    fn tem_voz(&self) -> bool{
        if self.idade > 1 {
            return true
        }
        return false;
    }
}
