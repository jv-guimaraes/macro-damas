#[derive(Debug, PartialEq)]
pub enum Estado {
    EsperandoJogador,
    PedraSelecionada(damas::Coord),
    AnimandoJogada,
    EsperandoComputador(f32),
    FimDoJogo,
}