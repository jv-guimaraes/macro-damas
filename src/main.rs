// #![allow(unused)]

use macroquad::{prelude::*};

mod tabuleiro;
mod util;
mod jogo;

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