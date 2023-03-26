#![allow(unused)]

use damas::Partida;
use macroquad::{prelude::*, rand::rand};

mod tabuleiro;
mod util;
mod jogo;
use jogo::Jogo;

fn window_conf() -> Conf {
    Conf {
        window_title: "Macro Damas".to_owned(),
        window_width: 650,
        window_height: 650,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut jogo = jogo::new_jogo().await;
    loop {
        jogo.run();
        next_frame().await
    }
}