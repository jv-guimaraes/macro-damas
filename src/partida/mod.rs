use damas::{Coord, Jogada};
use macroquad::{prelude::*};

mod animacao;
mod assets;
mod estado;
mod util;

use animacao::Animacao;
use assets::Assets;
use estado::Estado;
use util::{coord_para_tela, mouse_para_tabuleiro, tamanho_da_casa, uvec_to_coord};
use damasminimax::melhor_jogada_preta;

const TABULEIRO_INICIAL: [[char; 8]; 8] = [
    ['.', 'p', '.', 'p', '.', 'p', '.', 'p'],
    ['p', '.', 'p', '.', 'p', '.', 'p', '.'],
    ['.', 'p', '.', 'p', '.', 'p', '.', 'p'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['b', '.', 'b', '.', 'b', '.', 'b', '.'],
    ['.', 'b', '.', 'b', '.', 'b', '.', 'b'],
    ['b', '.', 'b', '.', 'b', '.', 'b', '.'],
];

const SEARCH_DEPTH: u32 = 9;
const BOT_DELAY: f32 = 0.15;

pub struct Partida {
    assets: Assets,
    partida: damas::Partida,
    animacao: Option<Animacao>,
    estado: Estado,
}

impl Partida {
    pub async fn iniciar() -> Self {
        Partida {
            assets: Assets::carregar().await,
            partida: damas::Partida::new(TABULEIRO_INICIAL),
            animacao: None,
            estado: Estado::EsperandoJogador,
        }
    }

    pub fn rodar(&mut self) {
        self.update();
        self.draw();
    }

    fn draw(&mut self) {
        self.desenhar_background();
        self.desenhar_texto_de_debug();
        match self.estado {
            Estado::AnimandoJogada => {
                self.desenhar_pedras(self.animacao.as_ref().unwrap().get_estado_inicial());
                self.animacao.as_mut().unwrap().desenhar();
            }
            Estado::PedraSelecionada(_) => {
                self.desenhar_pedras(self.partida.get_tabuleiro());
                self.desenhar_highlights_de_jogadas();
            }
            Estado::EsperandoJogador => {
                self.desenhar_pedras(self.partida.get_tabuleiro());
                self.desenhar_highlights_de_pedras();
            }
            _ => self.desenhar_pedras(self.partida.get_tabuleiro()),
        }
    }

    fn update(&mut self) {
        if self.estado == Estado::EsperandoJogador && is_mouse_button_released(MouseButton::Left) {
            let pos_tabuleiro = match mouse_para_tabuleiro() {
                Some(pos) => pos,
                None => return,
            };
            for jogada in self.partida.todas_jogadas_possiveis() {
                if jogada[0].origem() == uvec_to_coord(pos_tabuleiro) {
                    self.estado = Estado::PedraSelecionada(jogada[0].origem());
                    return;
                }
            }
        }

        if let Estado::PedraSelecionada(coord) = self.estado {
            if is_mouse_button_released(MouseButton::Left) {
                let pos_tabuleiro = match mouse_para_tabuleiro() {
                    Some(pos) => pos,
                    None => return,
                };
                for (ix, jogada) in self.partida.todas_jogadas_possiveis().iter().enumerate() {
                    if jogada[0].origem() == uvec_to_coord(pos_tabuleiro) {
                        self.estado = Estado::PedraSelecionada(jogada[0].origem());
                        return;
                    }
                    if jogada[0].origem() != coord {
                        continue;
                    }
                    for movimentos in jogada {
                        if movimentos.destino() == uvec_to_coord(pos_tabuleiro) {
                            self.estado = Estado::AnimandoJogada;
                            let textura = self
                                .pedra_to_textura(self.partida.pedra(jogada[0].origem()).unwrap());
                            self.animacao = Some(Animacao::new(
                                self.partida.get_tabuleiro(),
                                jogada.clone(),
                                textura,
                                self.assets.sound,
                            ));
                            self.partida.jogar(ix);
                            return;
                        }
                    }
                }
                self.estado = Estado::EsperandoJogador;
            }
        }

        if let Estado::AnimandoJogada = self.estado {
            if self.animacao.as_ref().unwrap().acabou() {
                if self.partida.ganhou() {
                    let ganhador = match self.animacao.as_ref().unwrap().get_pedra().é_branca() {
                        true => "Branco",
                        false => "Preto",
                    };
                    self.estado = Estado::Ganhou(ganhador.to_owned());
                } else if self.partida.empatou() {
                    self.estado = Estado::Empate;
                } else if self.partida.é_a_vez_do_branco() {
                    self.estado = Estado::EsperandoJogador;
                } else {
                    self.estado = Estado::EsperandoComputador(BOT_DELAY);
                }
                self.animacao = None;
            }
        }

        if let Estado::EsperandoComputador(delay) = &mut self.estado {
            *delay -= get_frame_time();
            if *delay <= 0.0 {
                let jogadas = self.partida.todas_jogadas_possiveis();
                let ix = melhor_jogada_preta(&self.partida, SEARCH_DEPTH);
                let textura =
                    self.pedra_to_textura(self.partida.pedra(jogadas[ix][0].origem()).unwrap());
                self.animacao = Some(Animacao::new(
                    self.partida.get_tabuleiro(),
                    jogadas[ix].clone(),
                    textura,
                    self.assets.sound,
                ));
                self.partida.jogar(ix);
                self.estado = Estado::AnimandoJogada;
            }
        }
    }

    fn desenhar_pedras(&self, tabuleiro: &[[damas::Casa; 8]; 8]) {
        #[allow(clippy::needless_range_loop)]
        for y in 0..8 {
            for x in 0..8 {
                if let damas::Casa::Ocupada(pedra) = tabuleiro[y][x] {
                    self.desenhar_pedra(x as u32, y as u32, pedra);
                }
            }
        }
    }

    fn desenhar_pedra(&self, x: u32, y: u32, tipo: damas::Pedra) {
        let coord = util::tabuleiro_para_tela(uvec2(x, y));
        let textura = self.pedra_to_textura(tipo);
        draw_texture_ex(
            textura,
            coord.x,
            coord.y,
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            DrawTextureParams {
                dest_size: Some(vec2(tamanho_da_casa().x, tamanho_da_casa().y)),
                ..Default::default()
            },
        )
    }

    fn desenhar_background(&self) {
        draw_texture_ex(
            self.assets.tabuleiro,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        )
    }

    fn pedra_to_textura(&self, pedra: damas::Pedra) -> Texture2D {
        match pedra {
            damas::Pedra::Branca => self.assets.branca,
            damas::Pedra::DamaBranca => self.assets.d_branca,
            damas::Pedra::Preta => self.assets.preta,
            damas::Pedra::DamaPreta => self.assets.d_preta,
        }
    }

    fn desenhar_texto_de_debug(&self) {
        draw_text(
            &format!("{:?}", self.estado),
            util::barra_vertical(),
            16.0,
            27.0,
            BLACK,
        );
    }

    fn desenhar_highlights_de_jogadas(&self) {
        if let Estado::PedraSelecionada(pedra) = self.estado {
            let jogadas = self.partida.todas_jogadas_possiveis();
            for jogada in jogadas.iter().filter(|x| x[0].origem() == pedra) {
                for movimento in jogada {
                    let cor = match movimento {
                        Jogada::Mover(_, _) => Color::new(0.0, 0.89, 0.19, 0.5),
                        Jogada::Capturar(_, _, _) => Color::new(1.0, 0.0, 0.0, 0.6),
                    };
                    self.desenhar_highlight(movimento.destino(), cor);
                }
            }
        }
    }

    fn desenhar_highlights_de_pedras(&self) {
        let mut jogadas = self.partida.todas_jogadas_possiveis().clone();
        jogadas.dedup_by_key(|x| x[0].origem());
        for jogada in jogadas {
            let branco = Color::new(1.0, 1.0, 1.0, 0.3);
            self.desenhar_highlight(jogada[0].origem(), branco);
        }
    }

    fn desenhar_highlight(&self, coord: Coord, cor: Color) {
        let pos = coord_para_tela(coord);
        let tamanho = tamanho_da_casa();
        // draw_rectangle(pos.x, pos.y, tamanho.x, tamanho.y, cor);
        draw_rectangle_lines(pos.x, pos.y, tamanho.x, tamanho.y, 10.0, cor);
    }
}
