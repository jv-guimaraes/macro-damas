use macroquad::prelude::*;

const BARRA_PORCENTAGEM_DA_TELA: f32 = 0.033796296;

pub fn barra_vertical() -> f32 {
    BARRA_PORCENTAGEM_DA_TELA * screen_width()
}

fn barra_horizontal() -> f32 {
    BARRA_PORCENTAGEM_DA_TELA * screen_height()
}

pub fn tamanho_da_casa() -> Vec2 {
    let largura = screen_width() - barra_vertical() * 2.0;
    let altura = screen_height() - barra_horizontal() * 2.0;
    vec2(largura / 8.0, altura / 8.0)
}

pub fn tabuleiro_para_tela(coord: UVec2) -> Vec2 {
    Vec2 {
        x: barra_vertical() + coord.x as f32 * tamanho_da_casa().x,
        y: barra_horizontal() + coord.y  as f32 * tamanho_da_casa().y,
    }
}

pub fn coord_para_tela(coord: damas::Coord) -> Vec2 {
    Vec2 {
        x: barra_vertical() + coord.x as f32 * tamanho_da_casa().x,
        y: barra_horizontal() + coord.y  as f32 * tamanho_da_casa().y,
    }
}

pub fn tela_para_tabuleiro(coord: Vec2) -> UVec2 {
    let x = coord.x - barra_vertical();
    let y = coord.y - barra_horizontal();
    UVec2 { x: (x / tamanho_da_casa().x) as u32, y: (y / tamanho_da_casa().y) as u32 }
}

pub fn mouse_para_tabuleiro() -> Option<UVec2> {
    let mouse = mouse_position();
    if mouse.0 < barra_vertical() || mouse.0 > screen_width() - barra_vertical() {
        return None;
    }
    if mouse.1 < barra_horizontal() || mouse.1 > screen_height() - barra_horizontal() {
        return None;
    }
    Some(tela_para_tabuleiro(vec2(mouse.0, mouse.1)))
}

pub fn uvec_to_coord(uvec: UVec2) -> damas::Coord {
    damas::Coord {
        x: uvec.x as i32,
        y: uvec.y as i32,
    }
}

pub fn vec_equals(first: Vec2, second: Vec2) -> bool {
    (first.x - second.x).abs() < 10.0 || (first.y - second.y).abs() < 10.0
}
