#![windows_subsystem = "windows"]
use macroquad::prelude::*;

mod partida;
use partida::Partida;

fn window_config() -> macroquad::window::Conf {
    macroquad::window::Conf {
        window_title: "Macro Damas".to_owned(),
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
        if is_key_down(KeyCode::Q) { std::process::exit(0) }
        tela_atual.rodar();
        next_frame().await
    }
}
