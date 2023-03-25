#![allow(unused)]
use damas::Partida;
use macroquad::{prelude::*, rand::rand};

mod tabuleiro;
use tabuleiro::Tabuleiro;
mod util;
use util::*;

const TABULEIRO: [[char; 8]; 8] = [
    ['.', 'p', '.', 'p', '.', 'p', '.', 'p'],
    ['p', '.', 'p', '.', 'p', '.', 'p', '.'],
    ['.', 'p', '.', 'p', '.', 'p', '.', 'p'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['b', '.', 'b', '.', 'b', '.', 'b', '.'],
    ['.', 'b', '.', 'b', '.', 'b', '.', 'b'],
    ['b', '.', 'b', '.', 'b', '.', 'b', '.'],
];

const COMPUTADOR_DELAY: f32 = 0.5;

fn window_conf() -> Conf {
    Conf {
        window_title: "Macro Damas".to_owned(),
        window_width: 550,
        window_height: 550,
        ..Default::default()
    }
}

#[derive(Debug, Clone, Copy)]
enum Vez {
    Humano,
    Computador,
}

impl Vez {
    fn passar(&mut self) {
        *self = match self {
            Vez::Humano => Vez::Computador,
            Vez::Computador => Vez::Humano,
        }
    }
}

struct Jogo {
    tabuleiro: Tabuleiro,
    partida: Partida,
    vez: Vez,
    computador_delay: f32,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut jogo = Jogo {
        tabuleiro: Tabuleiro::new().await,
        partida: damas::Partida::new(TABULEIRO),
        vez: Vez::Humano,
        computador_delay: COMPUTADOR_DELAY,
    };
    jogo.tabuleiro
        .ativar_highlights_brancos(jogo.partida.todas_jogadas_possiveis());

    loop {
        update(&mut jogo);
        draw(&jogo);
        next_frame().await
    }
}

fn draw(jogo: &Jogo) {
    jogo.tabuleiro.draw(jogo.partida.get_tabuleiro());
    let text = format!("Vez do {:?} {}", jogo.vez, posição_do_mouse_no_tabuleiro(),);
    draw_text(&text, 10.0, 15.0, 20.0, BLACK);
}

fn update(jogo: &mut Jogo) {
    if matches!(jogo.vez, Vez::Computador) {
        if jogo.computador_delay > 0.0 {
            jogo.computador_delay -= get_frame_time();
            return;
        }
        let jogadas = jogo.partida.todas_jogadas_possiveis();
        let jogada = rand() as usize % jogadas.len();
        jogo.partida.jogar(jogada);
        jogo.vez.passar();
        jogo.computador_delay = COMPUTADOR_DELAY;
        jogo.tabuleiro.ativar_highlights_brancos(jogo.partida.todas_jogadas_possiveis());
        return;
    }

    if is_mouse_button_released(MouseButton::Left) {
        if jogo.tabuleiro.há_pedra_selecionada() {
            let mut jogada = None;
            for (i, jogadas) in jogo.partida.todas_jogadas_possiveis().iter().enumerate() {
                let coord = jogadas[0].destino();
                let mouse = posição_do_mouse_no_tabuleiro();
                if cmp_coord_uvec(coord, mouse)
                    && cmp_coord_uvec(jogadas[0].origem(), jogo.tabuleiro.pedra_selecionada().unwrap())
                {
                    jogada = Some(i);
                    break;
                }
            }
            if let Some(jogada_ix) = jogada {
                jogo.partida.jogar(jogada_ix);
                jogo.tabuleiro.deselecionar_pedra();
                jogo.tabuleiro.ativar_highlights_brancos(jogo.partida.todas_jogadas_possiveis());
                jogo.vez.passar();
            }
        } else {
            for jogadas in jogo.partida.todas_jogadas_possiveis() {
                let coord = jogadas[0].origem();
                let mouse = posição_do_mouse_no_tabuleiro();
                if coord.x == mouse.x as i32 && coord.y == mouse.y as i32 {
                    jogo.tabuleiro
                        .selecionar_pedra(posição_do_mouse_no_tabuleiro());
                    jogo.tabuleiro.ativar_highlights_verdes(jogadas);
                }
            }
        }
    }

    if is_mouse_button_released(MouseButton::Right) {
        jogo.tabuleiro.deselecionar_pedra();
        jogo.tabuleiro
            .ativar_highlights_brancos(jogo.partida.todas_jogadas_possiveis());
    }
}
