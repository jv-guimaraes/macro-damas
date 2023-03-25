use damas::Coord;
use macroquad::prelude::*;

pub fn tamanho_tabuleiro_pixels() -> Vec2 {
    let width = screen_width() - (barra_vertical() * 2.0);
    let height = screen_height() - (barra_horizontal() * 2.0);
    vec2(width, height)
}

pub fn barra_horizontal() -> f32 {
    screen_height() * 0.03333333
}

pub fn barra_vertical() -> f32 {
    screen_width() * 0.03333333
}

pub fn remover_barras(pos: (f32, f32)) -> Vec2 {
    Vec2 {
        x: pos.0 - barra_horizontal(),
        y: pos.1 - barra_vertical(),
    }
}

pub fn tamanho_da_casa() -> Vec2 {
    let tela = tamanho_tabuleiro_pixels();
    vec2(tela.x / 8.0, tela.y / 8.0)
}

pub fn posição_do_mouse_no_tabuleiro() -> UVec2 {
    let pos_mouse_no_tab = remover_barras(mouse_position());
    let tamanho_casa = tamanho_da_casa();
    UVec2 {
        x: (pos_mouse_no_tab.x / tamanho_casa.x) as u32,
        y: (pos_mouse_no_tab.y / tamanho_casa.y) as u32,
    }
}

pub fn cmp_coord_uvec(coord: Coord, uvec: UVec2) -> bool {
    coord.x == uvec.x as i32 && coord.y == uvec.y as i32
}
