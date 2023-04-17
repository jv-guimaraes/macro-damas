use std::fmt::Debug;

use super::Pedra;

#[derive(Clone, Copy, PartialEq)]
pub enum Casa {
    Ocupada(Pedra),
    Vazia,
}

impl Casa {
    pub fn pedra(self) -> Option<Pedra> {
        match self {
            Casa::Ocupada(peça) => Some(peça),
            Casa::Vazia => None,
        }
    }

    pub fn é_vazia(self) -> bool {
        match self {
            Casa::Ocupada(_) => false,
            Casa::Vazia => true,
        }
    }
}

impl Debug for Casa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Casa::Ocupada(peça) => match peça {
                Pedra::Branca => write!(f, "x"),
                Pedra::DamaBranca => write!(f, "X"),
                Pedra::Preta => write!(f, "o"),
                Pedra::DamaPreta => write!(f, "O"),
            },
            Casa::Vazia => write!(f, "."),
        }
    }
}