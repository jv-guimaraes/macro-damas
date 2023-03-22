use damas::Casa;
use macroquad::prelude::*;

const PRETO: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 0.75 };
const BRANCO: Color = Color { r: 0.93, g: 0.93, b: 0.93, a: 1.0 };

pub struct Tabuleiro {
    textura: Texture2D,
    pedra: Texture2D,
    dama: Texture2D,
}

impl Tabuleiro {
    pub async fn new() -> Tabuleiro {
        Tabuleiro {
            textura: load_texture("assets\\tabuleiro.png").await.unwrap(),
            pedra: load_texture("assets\\pedra.png").await.unwrap(),
            dama: load_texture("assets\\dama.png").await.unwrap(),
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
                let pedra = casa.peça().unwrap();
                draw_texture_ex(
                    if pedra.é_dama() { self.dama } else { self.pedra },
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
}
