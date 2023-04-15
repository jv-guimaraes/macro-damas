use std::fmt::Debug;

#[derive(PartialEq)]
pub enum Estado {
    EsperandoJogador,
    PedraSelecionada(damas::Coord),
    AnimandoJogada,
    EsperandoComputador(f32),
    Ganhou(String),
    Empate, 
}

impl Debug for Estado {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EsperandoJogador => write!(f, "EsperandoJogador"),
            Self::PedraSelecionada(arg0) => f.debug_tuple("PedraSelecionada").field(arg0).finish(),
            Self::AnimandoJogada => write!(f, "AnimandoJogada"),
            Self::EsperandoComputador(arg0) => f.debug_tuple("EsperandoComputador").field(arg0).finish(),
            Self::Ganhou(ganhador) => write!(f, "{} ganhou!", ganhador),
            Self::Empate => write!(f, "Empate"),
        }
    }
}