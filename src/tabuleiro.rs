use damas::{Casa, Jogada};
use macroquad::prelude::*;

use crate::util::{
    barra_horizontal, barra_vertical, tamanho_da_casa,
};

const PRETO: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 0.75,
};
const BRANCO: Color = Color {
    r: 0.93,
    g: 0.93,
    b: 0.93,
    a: 1.0,
};

#[derive(Debug, Clone, Copy)]
enum Highlight {
    Nenhum,
    Branco,
    Verde,
}

pub struct Tabuleiro {
    textura: Texture2D,
    pedra: Texture2D,
    dama: Texture2D,
    highlights: [[Highlight; 8]; 8],
    pedra_selecionada: Option<UVec2>,
}

impl Tabuleiro {
    pub async fn new() -> Tabuleiro {
        let highlights = [[Highlight::Nenhum; 8]; 8];
        Tabuleiro {
            textura: load_texture("assets\\tabuleiro.png").await.unwrap(),
            pedra: load_texture("assets\\pedra.png").await.unwrap(),
            dama: load_texture("assets\\dama.png").await.unwrap(),
            highlights,
            pedra_selecionada: None,
        }
    }

    pub fn draw(&self, tabuleiro: &[[Casa; 8]; 8]) {
        self.desenhar_tabuleiro();
        self.desenhar_pedras(tabuleiro);
        self.desenhar_highlights();
    }

    pub fn desativar_highlights(&mut self) {
        for h in self.highlights.iter_mut().flatten() {
            *h = Highlight::Nenhum;
        }
    }

    pub fn ativar_highlights_brancos(&mut self, jogadas: &Vec<Vec<Jogada>>) {
        self.desativar_highlights();
        for jogada in jogadas {
            let coord = jogada[0].origem();
            self.highlights[coord.y as usize][coord.x as usize] = Highlight::Branco;
        }
    }

    pub fn ativar_highlights_verdes(&mut self, jogadas: &[Jogada]) {
        for jogada in jogadas {
            let dest = jogada.destino();
            self.highlights[dest.y as usize][dest.x as usize] = Highlight::Verde;
        }
    }

    fn desenhar_tabuleiro(&self) {
        draw_texture_ex(
            self.textura,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
    }

    fn desenhar_pedras(&self, tabuleiro: &[[Casa; 8]; 8]) {
        let offset_h = screen_width() * 0.032407;
        let offset_v = screen_height() * 0.032407;
        let width = (screen_width() - (offset_h * 2.0)) / 8.0;
        let height = (screen_height() - (offset_v * 2.0)) / 8.0;
        #[allow(clippy::needless_range_loop)]
        for y in 0..8 {
            for x in 0..8 {
                let casa = tabuleiro[y][x];
                if casa.é_vazia() {
                    continue;
                }
                let pedra = casa.peça().unwrap();
                draw_texture_ex(
                    if pedra.é_dama() {
                        self.dama
                    } else {
                        self.pedra
                    },
                    x as f32 * width + offset_h,
                    y as f32 * height + offset_v,
                    if pedra.é_preta() { PRETO } else { BRANCO },
                    DrawTextureParams {
                        dest_size: Some(vec2(width, height)),
                        ..Default::default()
                    },
                );
            }
        }
    }

    fn desenhar_highlights(&self) {
        for y in 0..8 {
            for x in 0..8 {
                let casa = self.highlights[y][x];
                let tamanho = tamanho_da_casa();
                let pos_x = barra_vertical() + tamanho.x * x as f32;
                let pos_y = barra_horizontal() + tamanho.y * y as f32;
                match casa {
                    Highlight::Nenhum => (),
                    Highlight::Branco => draw_rectangle(
                        pos_x,
                        pos_y,
                        tamanho.x,
                        tamanho.y,
                        Color::new(1.0, 1.0, 1.0, 0.2),
                    ),
                    Highlight::Verde => draw_rectangle(
                        pos_x,
                        pos_y,
                        tamanho.x,
                        tamanho.y,
                        Color::new(0.0, 1.0, 0.0, 0.35),
                    ),
                }
            }
        }
    }

    pub fn selecionar_pedra(&mut self, pedra: UVec2) {
        self.pedra_selecionada = Some(pedra);
    }

    pub fn deselecionar_pedra(&mut self) {
        self.pedra_selecionada = None;
    }

    pub fn há_pedra_selecionada(&self) -> bool {
        matches!(self.pedra_selecionada, Some(_))
    }

    pub fn pedra_selecionada(&self) -> Option<UVec2> {
        self.pedra_selecionada
    }
}
