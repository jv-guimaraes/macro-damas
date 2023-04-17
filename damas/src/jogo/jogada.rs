use super::coord::Coord;

#[derive(PartialEq, Clone, Copy)]
pub enum Jogada {
    Mover(Coord, Coord),              // (origem, destino)
    Capturar(Coord, Coord, Coord),       // (origem, comida, destino)
}

impl std::fmt::Debug for Jogada {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} -> {:?}", self.origem(), self.destino())
    }
}

impl Jogada {
    pub fn origem(&self)-> Coord {
        match self {
            Jogada::Mover(o, _) => *o,
            Jogada::Capturar(o, _, _) => *o,
        }
    }

    pub fn destino(&self)-> Coord {
        match self {
            Jogada::Mover(_, d) => *d,
            Jogada::Capturar(_, _, d) => *d,
        }
    }

    pub fn captura(&self) -> Coord {
        if let Jogada::Capturar(_, c, _) = self {
            *c
        } else {
            panic!("{:?} não é uma captura!", self);
        }
    }
}