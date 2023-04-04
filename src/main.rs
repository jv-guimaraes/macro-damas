#![allow(unused)]
use macroquad::prelude::*;

mod partida;
use partida::Partida;

fn window_config() -> macroquad::window::Conf {
    macroquad::window::Conf {
        window_width: 600,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut tela_atual = Partida::iniciar().await;
    loop {
        clear_background(GRAY);
        tela_atual.rodar();
        next_frame().await
    }
}
