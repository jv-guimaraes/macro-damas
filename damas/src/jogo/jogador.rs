#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Jogador {
    Branco,
    Preto,
}

impl Jogador {
    pub fn é_branco(self) -> bool {
        match self {
            Jogador::Branco => true,
            Jogador::Preto => false,
        }
    }
}