#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pedra {
    Branca,
    DamaBranca,
    Preta,
    DamaPreta,
}

impl Pedra {
    pub fn é_branca(self) -> bool {
        matches!(self, Pedra::Branca | Pedra::DamaBranca)
    }

    pub fn é_preta(self) -> bool {
        matches!(self, Pedra::Preta | Pedra::DamaPreta)
    }

    pub fn é_dama(self) -> bool {
        matches!(self, Pedra::DamaPreta | Pedra::DamaBranca)
    }

    pub(crate) fn dama(self) -> Self {
        if self.é_branca() {
            Pedra::DamaBranca
        } else {
            Pedra::DamaPreta
        }
    }
}