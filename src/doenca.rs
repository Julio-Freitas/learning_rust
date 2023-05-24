use std::fmt;
pub struct Doenca {
    pub nome: String,
    pub sintomas: Vec<String>,
    pub causa: String,
    pub tratamento: String,
}

impl Doenca {
    pub fn new(nome: String, sintomas: Vec<String>, causa: String, tratamento: String) -> Doenca {
        Doenca {
            nome,
            sintomas,
            causa,
            tratamento,
        }
    }

    pub fn get_nome(&self) -> &String {
        &self.nome
    }

    pub fn get_sintomas(&self) -> &Vec<String> {
        &self.sintomas
    }

    pub fn get_causa(&self) -> &String {
        &self.causa
    }

    pub fn get_tratamento(&self) -> &String {
        &self.tratamento
    }
}

impl fmt::Display for Doenca {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\nNome:{}\nSintomaas:{:?}\nCausa:{}\nTratamento:{}\n",
            self.get_nome(),
            self.get_sintomas(),
            self.get_causa(),
            self.get_tratamento()
        )
    }
}
