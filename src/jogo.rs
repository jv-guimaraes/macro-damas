#![allow(unused)]
use damas::Partida;
use macroquad::{prelude::*, rand::rand};

use crate::tabuleiro::Tabuleiro;
use crate::util::*;

const TABULEIRO: [[char; 8]; 8] = [
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', 'p', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', 'p', '.', '.'],
    ['.', '.', 'p', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', 'p', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', 'B', '.', '.', '.', '.', '.'],
];

const COMPUTADOR_DELAY: f32 = 0.5;



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

pub struct Jogo {
    tabuleiro: Tabuleiro,
    partida: Partida,
    vez: Vez,
    computador_delay: f32,
    acabou: bool,
}

impl Jogo {
    fn draw(&self) {
        self.tabuleiro.draw(self.partida.get_tabuleiro());
        let text = if self.acabou {
            " Game over!".to_owned()
        } else {
            format!(" Vez do {:?} {}", self.vez, posição_do_mouse_no_tabuleiro())
        };
        draw_text(&text, 10.0, 15.0, 20.0, BLACK);
    }

    fn update(&mut self) {
        if self.acabou { return; };

        if matches!(self.vez, Vez::Computador) {
            if self.computador_delay > 0.0 {
                self.computador_delay -= get_frame_time();
                return;
            }
            let jogadas = self.partida.todas_jogadas_possiveis();
            let jogada = rand() as usize % jogadas.len();
            if matches!(self.partida.jogar(jogada), damas::Resultado::FimDoJogo(_)) {
                self.acabou = true;
                return; 
            }
            self.vez.passar();
            self.computador_delay = COMPUTADOR_DELAY;
            self.tabuleiro.ativar_highlights_brancos(self.partida.todas_jogadas_possiveis());
            return;
        }
    
        if is_mouse_button_released(MouseButton::Left) {
            if self.tabuleiro.há_pedra_selecionada() {
                let mut jogada = None;
                for (i, jogadas) in self.partida.todas_jogadas_possiveis().iter().enumerate() {
                    for coord in jogadas.iter().map(|x| x.destino()) {
                        let mouse = posição_do_mouse_no_tabuleiro();
                        if cmp_coord_uvec(coord, mouse)
                            && cmp_coord_uvec(jogadas[0].origem(), self.tabuleiro.pedra_selecionada().unwrap())
                        {
                            jogada = Some(i);
                            break;
                        }
                    }
                }
                if let Some(jogada_ix) = jogada {
                    self.tabuleiro.deselecionar_pedra();
                    self.tabuleiro.ativar_highlights_brancos(self.partida.todas_jogadas_possiveis());
                    self.vez.passar();
                    if matches!(self.partida.jogar(jogada_ix), damas::Resultado::FimDoJogo(_)) {
                        self.acabou = true;
                        return; 
                    }
                }
            } else {
                for jogadas in self.partida.todas_jogadas_possiveis() {
                    let coord = jogadas[0].origem();
                    let mouse = posição_do_mouse_no_tabuleiro();
                    if coord.x == mouse.x as i32 && coord.y == mouse.y as i32 {
                        self.tabuleiro
                            .selecionar_pedra(posição_do_mouse_no_tabuleiro());
                        self.tabuleiro.ativar_highlights_verdes(jogadas);
                    }
                }
            }
        }
    
        if is_mouse_button_released(MouseButton::Right) {
            self.tabuleiro.deselecionar_pedra();
            self.tabuleiro
                .ativar_highlights_brancos(self.partida.todas_jogadas_possiveis());
        }
    }

    pub fn run(&mut self) {
        self.update();
        self.draw();
    }
}

pub async fn new_jogo() -> Jogo {
    let mut jogo = Jogo {
        tabuleiro: Tabuleiro::new().await,
        partida: damas::Partida::new(TABULEIRO),
        vez: Vez::Humano,
        computador_delay: COMPUTADOR_DELAY,
        acabou: false,
    };
    jogo.tabuleiro.ativar_highlights_brancos(jogo.partida.todas_jogadas_possiveis());
    jogo
}
