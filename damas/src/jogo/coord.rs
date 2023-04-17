use std::{fmt::{Debug}, ops::Add};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn diagonais_de_captura(self) -> Vec<Coord> {
        let mut diagonais = vec![];
        for (i, j) in [(1, 1), (-1, -1),(1, -1), (-1, 1)] {
            let (x, y) = (self.x + i, self.y + j);
            if é_valida(x, y) && é_valida(x + i, y + j) {
                diagonais.push(c(x, y));
            }
        }
        diagonais
    }

    pub fn diagonais_superiores(self) -> Vec<Coord> {
        let mut diagonais: Vec<Coord> = Vec::new();
        if self.y == 0 {
            return diagonais;
        };

        let y = self.y - 1;
        if self.x > 0 {
            diagonais.push(c(self.x - 1, y));
        }
        if self.x < 7 {
            diagonais.push(c(self.x + 1, y));
        }
        diagonais
    }

    pub fn diagonais_inferiores(self) -> Vec<Coord> {
        let mut diagonais: Vec<Coord> = Vec::new();
        if self.y == 7 {
            return diagonais;
        };

        let y = self.y + 1;
        if self.x > 0 {
            diagonais.push(c(self.x - 1, y));
        }
        if self.x < 7 {
            diagonais.push(c(self.x + 1, y));
        }
        diagonais
    }

    pub fn diagonais_rainha(self) -> Vec<Coord> {
        let mut diagonais: Vec<Coord> = Vec::new();
        for (i, j) in [(1, 1), (-1, -1), (1, -1), (-1, 1)] {
            let (mut x, mut y) = (self.x, self.y);
            x += i; y += j;
            while é_valida(x, y) {
                diagonais.push(c(x, y));
                x += i; y += j;
            }
        }
        diagonais
    }

    pub fn distancia(self, other: Self) -> Coord {
        Coord { x: other.x - self.x, y: other.y - self.y}
    }

    pub fn vezes(self, fator: i32) -> Coord {
        Coord { x: self.x * fator, y: self.y * fator }
    }

    pub fn normal(self) -> Coord {
        Coord { x: self.x / self.x.abs(), y: self.y / self.y.abs() }
    }

    pub fn é_valida(self) -> bool {
        if !(0..=7).contains(&self.x) {
            return false;
        }
        if !(0..=7).contains(&self.y) {
            return false;
        }
        true
    }

    pub fn está_na_faixa_de_damas(self) -> bool {
        self.y == 0 || self.y == 7
    }
}

impl Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

pub fn c(x: i32, y: i32) -> Coord {
    Coord { x, y }
}

fn é_valida(x: i32, y: i32) -> bool {
    if !(0..=7).contains(&x) {
        return false;
    }
    if !(0..=7).contains(&y) {
        return false;
    }
    true
}

#[test]
fn testar_diagonais() {
    let coord = c(2, 5);
    assert_eq!(coord.diagonais_superiores(), vec![c(1, 4), c(3, 4)]);
    assert_eq!(coord.diagonais_inferiores(), vec![c(1, 6), c(3, 6)]);

    let coord = c(0, 0);
    assert_eq!(coord.diagonais_superiores(), vec![]);
    assert_eq!(coord.diagonais_inferiores(), vec![c(1, 1)]);
    
    let coord = c(7, 7);
    assert_eq!(coord.diagonais_superiores(), vec![c(6, 6)]);
    assert_eq!(coord.diagonais_inferiores(), vec![]);
    
    let coord = c(7, 4);
    assert_eq!(coord.diagonais_superiores(), vec![c(6, 3)]);
    assert_eq!(coord.diagonais_inferiores(), vec![c(6, 5)]);

    let coord = c(3, 3);
    assert_eq!(
        coord.diagonais_rainha(),
        vec![c(4, 4),c(5, 5), c(6, 6), c(7, 7),
            c(2, 2), c(1, 1), c(0, 0),
            c(4, 2), c(5, 1), c(6, 0),
            c(2, 4), c(1, 5), c(0, 6)]
    );

    let coord = c(6, 1);
    assert_eq!(
        coord.diagonais_rainha(),
        vec![c(7, 2), c(5, 0), c(7, 0),
            c(5, 2), c(4, 3), c(3, 4),
            c(2, 5), c(1, 6), c(0, 7)]
    );

    let coord = c(7, 7);
    assert_eq!(coord.diagonais_rainha(), vec![c(6,6), c(5,5), c(4,4), c(3,3), c(2,2), c(1,1), c(0,0)]);

    let coord = c(3, 3);
    assert_eq!(coord.diagonais_de_captura(), vec![c(4,4), c(2,2), c(4,2), c(2,4)]);

    let coord = c(7, 7);
    assert_eq!(coord.diagonais_de_captura(), vec![c(6, 6)]);


}

#[test]
fn testar_aritimetica() {
    let coord = c(7, 7);
    assert_eq!(coord.distancia(c(6, 6)).vezes(2), c(-2, -2));
    let coord = c(2, 2);
    assert_eq!(coord.normal(), c(1, 1));
    let coord = c(-3, -3);
    assert_eq!(coord.normal(), c(-1, -1));
    assert!(c(1, 1) == c(1, 1));
}
