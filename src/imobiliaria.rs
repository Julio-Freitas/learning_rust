pub struct Endereco {
    pub rua: String,
    pub cep: u64,
    pub cidade: String,
    pub bairro: String,
}

pub struct Imovel {
    pub endereco: Endereco,
    pub banheiros: u8,
    pub quartos: u8,
    pub metragem: f64,
    pub preco: f64,
}

pub struct Imobiliaria {
    pub endereco: Endereco,
    pub imoveis: Vec<Imovel>,
}

impl Imobiliaria {
    pub fn novo_imovel(
        &mut self,
        endereco: Endereco,
        banheiros: u8,
        quartos: u8,
        metragem: f64,
        preco: f64,
    ) {
        let imovel = Imovel {
            banheiros,
            endereco,
            metragem,
            quartos,
            preco,
        };
        self.imoveis.push(imovel)
    }

    pub fn listar_imoveis(&self) {
        for imovel in &self.imoveis {
            println!(
                "Endereco: Rua:{}, cidade:{}",
                imovel.endereco.rua, imovel.endereco.cidade
            );

            println!(
                "Banehiro: {}\nQuartos: {}, metragem:{}\npreço:R${}",
                imovel.banheiros, imovel.quartos, imovel.metragem, imovel.preco
            );
        }
    }
}
