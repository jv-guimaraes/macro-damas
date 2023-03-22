use macroquad::prelude::*;

pub struct Tabuleiro {
    textura: Texture2D,
}

impl Tabuleiro {
    pub async fn new() -> Tabuleiro {
        Tabuleiro {
            textura: load_texture("assets\\tabuleiro.png").await.unwrap()
        }
    }

    pub fn draw(&self) {
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
}
