#![allow(unused)]
use macroquad::prelude::*;

mod tabuleiro;
use tabuleiro::Tabuleiro;

fn window_conf() -> Conf {
    Conf {
        window_title: "Macro Damas".to_owned(),
        window_width: 550,
        window_height: 550,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let tabuleiro = Tabuleiro::new().await;
    let jogo = damas::Jogo::default();
    println!("{jogo}");
    loop {
        tabuleiro.draw();
        next_frame().await
    }
}
