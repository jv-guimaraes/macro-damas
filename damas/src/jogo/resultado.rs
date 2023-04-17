use super::jogador::Jogador;

#[derive(Debug)]
pub enum Resultado {
    Falha,          // Jogada invalida. Não passa o turno nem mexe no tabuleiro
    Sucesso,        // Jogada válida e passa o turno. Não tem mais possiveis captura
    FimDoJogo(Option<Jogador>), // Jogada válida e fim do jogo. Retorna o ganhador
}