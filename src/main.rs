#![allow(unused)]
use macroquad::prelude::*;

mod tabuleiro;
use tabuleiro::Tabuleiro;

const TABULEIRO: [[char; 8]; 8] = [
    ['.', 'p', '.', 'p', '.', 'p', '.', 'p'],
    ['p', '.', 'p', '.', 'p', '.', 'p', '.'],
    ['.', 'p', '.', '.', '.', 'p', '.', 'p'],
    ['.', '.', '.', '.', 'p', '.', '.', '.'],
    ['.', '.', '.', '.', '.', 'b', '.', '.'],
    ['b', '.', 'b', '.', '.', '.', 'b', '.'],
    ['.', 'b', '.', 'b', '.', 'b', '.', 'b'],
    ['b', '.', 'b', '.', 'b', '.', 'b', '.'],
];

fn window_conf() -> Conf {
    Conf {
        window_title: "Macro Damas".to_owned(),
        window_width: 600,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let tabuleiro = Tabuleiro::new().await;
    let jogo = damas::Jogo::new(TABULEIRO);
    loop {
        tabuleiro.draw(jogo.get_tabuleiro());
        next_frame().await
    }
}
