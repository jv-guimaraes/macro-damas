use macroquad::{prelude::*, audio::{load_sound, Sound}};

pub struct Assets {
    pub tabuleiro: Texture2D,
    pub branca: Texture2D,
    pub preta: Texture2D,
    pub d_branca: Texture2D,
    pub d_preta: Texture2D,
    pub sound: Sound,
}

impl Assets {
    pub async fn carregar() -> Self {
        Assets {
            tabuleiro: load_texture("assets\\tabuleiro.png").await.unwrap(),
            branca: load_texture("assets\\branca.png").await.unwrap(),
            preta: load_texture("assets\\preta.png").await.unwrap(),
            d_branca: load_texture("assets\\dama-branca.png").await.unwrap(),
            d_preta: load_texture("assets\\dama-preta.png").await.unwrap(),
            sound: load_sound("assets\\piece_sound.wav").await.unwrap(),
        }
    }
}