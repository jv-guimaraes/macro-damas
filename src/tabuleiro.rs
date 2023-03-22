use damas::Casa;
use macroquad::prelude::*;

pub struct Tabuleiro {
    textura: Texture2D,
    branca: Texture2D,
    preta: Texture2D,
}

impl Tabuleiro {
    pub async fn new() -> Tabuleiro {
        Tabuleiro {
            textura: load_texture("assets\\tabuleiro.png").await.unwrap(),
            branca: load_texture("assets\\pedra_branca.png").await.unwrap(),
            preta: load_texture("assets\\pedra_preta.png").await.unwrap(),
        }
    }

    pub fn draw(&self, tabuleiro: &[[Casa; 8]; 8]) {
        self.desenhar_tabuleiro();
        self.desenhar_pedras(tabuleiro);
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
                let peça = casa.peça().unwrap();
                draw_texture_ex(
                    if peça.é_preta() { self.preta } else { self.branca },
                    x as f32 * width + offset_h,
                    y as f32 * height + offset_v,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(width, height)),
                        ..Default::default()
                    },
                );
            }
        }
    }
}
