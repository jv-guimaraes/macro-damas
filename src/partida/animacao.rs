use std::collections::VecDeque;

use interpolation::Ease;
use macroquad::{
    audio::{load_sound, PlaySoundParams, Sound},
    prelude::*,
};

use super::util::{self, coord_para_tela, tamanho_da_casa};

pub struct Animacao {
    pub estado_inicial: [[damas::Casa; 8]; 8],
    jogadas: VecDeque<damas::Jogada>,
    pos: Vec2,
    textura: Texture2D,
    speed: f32,
    sound: Sound,
}

impl Animacao {
    pub fn new(
        estado_inicial: &[[damas::Casa; 8]; 8],
        jogadas: Vec<damas::Jogada>,
        textura: Texture2D,
        sound: Sound,
    ) -> Self {
        let origem = jogadas[0].origem();
        let pedra = estado_inicial[origem.y as usize][origem.x as usize]
            .pedra()
            .unwrap();
        let mut estado_inicial = *estado_inicial;
        estado_inicial[origem.y as usize][origem.x as usize] = damas::Casa::Vazia;
        Animacao {
            estado_inicial,
            jogadas: VecDeque::from(jogadas),
            pos: util::coord_para_tela(origem),
            textura,
            speed: 20.0,
            sound,
        }
    }

    pub fn desenhar(&mut self) {
        draw_text("Animating...", screen_width() - 120.0, 15.0, 20.0, BLACK);
        let jogada = self.jogadas[0];
        let destino = coord_para_tela(jogada.destino());
        self.pos = self
            .pos
            .lerp(destino, (self.speed * get_frame_time()).quadratic_in_out());
        if util::vec_equals(self.pos, coord_para_tela(jogada.destino())) {
            macroquad::audio::play_sound(self.sound, PlaySoundParams::default());
            self.jogadas.pop_front();
        }
        draw_texture_ex(
            self.textura,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(util::tamanho_da_casa()),
                ..Default::default()
            },
        );
    }

    pub fn acabou(&self) -> bool {
        self.jogadas.is_empty()
    }

    pub fn get_estado_inicial(&self) -> &[[damas::Casa; 8]; 8] {
        &self.estado_inicial
    }
}
