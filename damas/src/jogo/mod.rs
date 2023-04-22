use itertools::Itertools;
use std::fmt::Display;

pub mod coord;
pub mod jogador;
pub mod casa;
pub mod pedra;
pub mod jogada;
pub mod resultado;

use self::coord::{c, Coord};
use self::jogada::Jogada;
use self::jogador::Jogador;
use self::resultado::Resultado;
use self::casa::Casa;
use self::pedra::Pedra;

const TABULEIRO_INICIAL_CHARS: [[char; 8]; 8] = [
    ['.', 'p', '.', 'p', '.', 'p', '.', 'p'],
    ['p', '.', 'p', '.', 'p', '.', 'p', '.'],
    ['.', 'p', '.', 'p', '.', 'p', '.', 'p'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['b', '.', 'b', '.', 'b', '.', 'b', '.'],
    ['.', 'b', '.', 'b', '.', 'b', '.', 'b'],
    ['b', '.', 'b', '.', 'b', '.', 'b', '.'],
];

#[derive(Debug, Clone)]
pub struct Partida {
    tabuleiro: [[Casa; 8]; 8],
    vez: Jogador,
    jogadas: Vec<Vec<Jogada>>,
    contador_empate: f32,
}

impl Default for Partida {
    fn default() -> Self {
        // Construir tabuleiro inicial
        let mut tabuleiro = [[Casa::Vazia; 8]; 8];
        for y in 0..tabuleiro.len() {
            for x in 0..tabuleiro.len() {
                match TABULEIRO_INICIAL_CHARS[y][x] {
                    'p' => tabuleiro[y][x] = Casa::Ocupada(Pedra::Preta),
                    'b' => tabuleiro[y][x] = Casa::Ocupada(Pedra::Branca),
                    'P' => tabuleiro[y][x] = Casa::Ocupada(Pedra::DamaPreta),
                    'B' => tabuleiro[y][x] = Casa::Ocupada(Pedra::DamaBranca),
                    '.' => (),
                    c => panic!("{c} não é uma peça válida!"),
                }
            }
        }
        // Começar o jogo com a peça branca
        let mut p = Partida {
            tabuleiro,
            vez: Jogador::Branco,
            jogadas: vec![],
            contador_empate: 0.0,
        };
        p.jogadas = p._todas_jogadas_possiveis();
        p
    }
}

impl Display for Partida {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        buffer.push_str("   0  1  2  3  4  5  6  7\n");
        for y in 0..8 {
            buffer.push_str(&format!("{y} "));
            for x in 0..8 {
                match self.tabuleiro[y][x] {
                    Casa::Ocupada(peça) => match peça {
                        Pedra::Branca => buffer.push_str(" x "),
                        Pedra::Preta => buffer.push_str(" o "),
                        Pedra::DamaBranca => buffer.push_str(" X "),
                        Pedra::DamaPreta => buffer.push_str(" O "),
                    },
                    Casa::Vazia => buffer.push_str(" . "),
                }
            }
            buffer.push('\n');
        }
        write!(f, "{}", &buffer)
    }
}

impl Partida {
    pub fn jogar(&mut self, jogada: usize) -> Resultado {
        // Checar se a jogada escolhida é válida
        let todas_jogadas = self._todas_jogadas_possiveis();
        let jogada = todas_jogadas.get(jogada);
        if jogada.is_none() {
            return Resultado::Falha;
        }

        // Executar  a jogada
        let jogada = jogada.unwrap();
        let pedra_usada = self.pedra(jogada[0].origem()).unwrap();
        for jogada in jogada {
            self.executar_jogada(*jogada);
        }

        // Checar se deve virar dama
        let casa_final = jogada.last().unwrap().destino();
        if casa_final.está_na_faixa_de_damas() {
            let peça = self.pedra(casa_final).unwrap();
            if peça.é_branca() && (casa_final.y == 0) || peça.é_preta() && (casa_final.y == 7) {
                *self.casa_mut(casa_final) = Casa::Ocupada(self.pedra(casa_final).unwrap().dama());
            }
        }

        // Atualizar contador de empate
        if self.duas_damas() || pedra_usada.é_dama() {
            self.contador_empate += 0.5;
        } else {
            self.contador_empate = 0.0;
        }

        // Checar se acabou o jogo
        if self.ganhou() {
            return Resultado::FimDoJogo(Some(self.vez));
        }
        if self.empatou() {
            return Resultado::FimDoJogo(None);
        }

        self.passar_turno();

        // Caso n tenha nenhuma jogada disponível, então o jogador oposto ganhou
        if self.jogadas.is_empty() {
            let ganhador = Some(self.vez.oposto());
            return Resultado::FimDoJogo(ganhador);
        }

        Resultado::Sucesso
    }

    pub fn todas_jogadas_possiveis(&self) -> &Vec<Vec<Jogada>> {
        &self.jogadas
    }

    fn _todas_jogadas_possiveis(&self) -> Vec<Vec<Jogada>> {
        let capturas = self.todas_capturas_possiveis();
        if capturas.is_empty() {
            self.todos_movimentos_possiveis()
        } else {
            capturas
        }
    }

    pub fn get_vez(&self) -> Jogador {
        self.vez
    }

    pub fn get_tabuleiro(&self) -> &[[Casa; 8]; 8] {
        &self.tabuleiro
    }

    pub fn new(tabuleiro: [[char; 8]; 8]) -> Self {
        // Construir tabuleiro inicial
        let mut tab = [[Casa::Vazia; 8]; 8];
        for y in 0..tab.len() {
            for x in 0..tab.len() {
                match tabuleiro[y][x] {
                    'p' => tab[y][x] = Casa::Ocupada(Pedra::Preta),
                    'b' => tab[y][x] = Casa::Ocupada(Pedra::Branca),
                    'P' => tab[y][x] = Casa::Ocupada(Pedra::DamaPreta),
                    'B' => tab[y][x] = Casa::Ocupada(Pedra::DamaBranca),
                    '.' => (),
                    c => panic!("{c} não é uma peça válida!"),
                }
            }
        }
        // Começar o jogo com a peça branca
        let mut p = Partida {
            tabuleiro: tab,
            vez: Jogador::Branco,
            jogadas: vec![],
            contador_empate: 0.0,
        };
        p.jogadas = p._todas_jogadas_possiveis();
        p
    }

    fn executar_jogada(&mut self, jogada: Jogada) {
        match jogada {
            Jogada::Mover(origem, destino) => self.mover_sem_checar(origem, destino),
            Jogada::Capturar(origem, captura, destino) => {
                self.mover_sem_checar(origem, destino);
                *self.casa_mut(captura) = Casa::Vazia;
            },
        }
    }

    fn mover_sem_checar(&mut self, origem: Coord, destino: Coord) {
        *self.casa_mut(destino) = self.casa(origem);
        *self.casa_mut(origem) = Casa::Vazia;
    }

    fn calcular_capturas(&self, origem: Coord) -> Vec<Vec<Jogada>> {
        let mut stack: Vec<Jogada> = vec![];
        let mut sequencias: Vec<Vec<Jogada>> = vec![];
        let peça = self.pedra(origem);
        if peça.is_none() { return vec![vec![]];}
        let mut clone_sem_origem = self.clone();
        *clone_sem_origem.casa_mut(origem) = Casa::Vazia;
        clone_sem_origem.calcular_capturas_recursivamente(origem, &mut stack, &mut sequencias, peça.unwrap());
        // Filtrar apenas as maiores cadeias
        if sequencias.is_empty() { return vec![vec![]] }
        let maior_cadeia = sequencias.iter().max_by_key(|x| x.len()).unwrap().len();
        sequencias.into_iter().filter(|x| x.len() == maior_cadeia).collect()
        // Sem filtrar
        // sequencias
    }

    fn calcular_capturas_recursivamente(&self, origem: Coord, stack: &mut Vec<Jogada>, sequencias: &mut Vec<Vec<Jogada>>, peça: Pedra) {
        // println!("origem: {:?}", origem);
        // println!("stack: {:?}", stack);
        // println!("sequencias: {:?}", sequencias.len());
        // println!("--------------------------------------------------------------------");
        'a: for captura in self.capturas_imediatas(origem, peça) {
            // println!("{:?}", captura);
            // if !stack.is_empty() && stack.last().unwrap().origem() == captura.destino() {
            //     continue;
            // }
            // if stack.contains(&captura) { continue; }
            for captura_anterior in stack.iter() {
                if captura_anterior.captura() == captura.captura() {
                    continue 'a;
                }
            }
            stack.push(captura);
            // println!("{:?}", stack);
            self.calcular_capturas_recursivamente(captura.destino(), stack, sequencias, peça)
        }
        if !stack.is_empty() {
            sequencias.push(stack.clone());
        }
        stack.pop();
        // println!("{:?}", stack);
    }

    fn capturas_imediatas(&self, origem: Coord, peça: Pedra) -> Vec<Jogada> {
        match peça {
            Pedra::Branca | Pedra::Preta => self.capturas_imediatas_peão(origem),
            Pedra::DamaBranca | Pedra::DamaPreta => self.capturas_imediatas_dama(origem),
        }
    }

    fn capturas_imediatas_dama(&self, origem: Coord) -> Vec<Jogada> {
        let mut capturas = vec![];
        for dir in [c(1, 1), c(-1, -1), c(1, -1), c(-1, 1)] {
            let mut atual = origem + dir;
            while atual.é_valida() && self.casa(atual).é_vazia() {
                atual = atual + dir;
            }
            if atual.é_valida() && !self.é_a_vez_de(self.pedra(atual).unwrap()) {
                let mut pulo = (atual) + (origem.distancia(atual).normal());
                if atual.é_valida() && pulo.é_valida() && self.casa(pulo).é_vazia() {
                    while pulo.é_valida() && self.casa(pulo).é_vazia() {
                        capturas.push(Jogada::Capturar(origem, atual, pulo));
                        pulo = pulo + dir;
                    }
                }
            }
        }
        capturas
    }

    fn capturas_imediatas_peão(&self, origem: Coord) -> Vec<Jogada> {
        let mut capturas = vec![];
        for vizinho in origem.diagonais_de_captura() {
            if let Casa::Ocupada(peça) = self.casa(vizinho) {
                if self.é_a_vez_de(peça) { continue; }
                let destino = vizinho + origem.distancia(vizinho);
                if self.casa(destino).é_vazia() {
                    capturas.push(Jogada::Capturar(origem, vizinho, destino));
                }
            }
        }
        capturas
    }

    fn calcular_movimentos(&self, origem: Coord) -> Vec<Jogada> {
        match self.pedra(origem).unwrap() {
            Pedra::Branca | Pedra::Preta => self.movimentos_peão(origem),
            Pedra::DamaBranca | Pedra::DamaPreta => self.movimentos_dama(origem),
        }
    }

    fn movimentos_dama(&self, origem: Coord) -> Vec<Jogada> {
        let mut movimentos = vec![];
        for dir in [c(1, 1), c(-1, -1), c(1, -1), c(-1, 1)] {
            let mut atual = origem + dir;
            while atual.é_valida() && self.casa(atual).é_vazia() {
                movimentos.push(Jogada::Mover(origem, atual));
                atual = atual + dir;
            }
        }
        movimentos
    }

    fn movimentos_peão(&self, origem: Coord) -> Vec<Jogada> {
        let diagonais = match self.pedra(origem).unwrap() {
            Pedra::Branca => origem.diagonais_superiores(),
            Pedra::Preta => origem.diagonais_inferiores(),
            _ => panic!(),
        };
        diagonais
            .into_iter()
            .filter(|c| self.casa(*c).é_vazia())
            .map(|c| Jogada::Mover(origem, c))
            .collect()
    }

    pub fn pedra(&self, coord: Coord) -> Option<Pedra> {
        self.casa(coord).pedra()
    }

    pub fn casa(&self, coord: Coord) -> Casa {
        self.tabuleiro[coord.y as usize][coord.x as usize]
    }

    fn casa_mut(&mut self, coord: Coord) -> &mut Casa {
        &mut self.tabuleiro[coord.y as usize][coord.x as usize]
    }

    fn é_a_vez_de(&self, peça: Pedra) -> bool {
        if peça.é_branca() && self.vez == Jogador::Preto {
            return false;
        }
        if peça.é_preta() && self.vez == Jogador::Branco {
            return false;
        }
        true
    }

    fn passar_turno(&mut self) {
        self.vez = match self.vez {
            Jogador::Branco => Jogador::Preto,
            Jogador::Preto => Jogador::Branco,
        };
        self.jogadas = self._todas_jogadas_possiveis();
    }

    fn peças_da_cor_atual(&self) -> Vec<Coord> {
        let mut peças = vec![];
        for y in 0..8 {
            for x in 0..8 {
                if let Casa::Ocupada(peça) = self.tabuleiro[y][x] {
                    if self.é_a_vez_de(peça) {
                        peças.push(c(x as i32, y as i32));
                    }
                }
            }
        }
        peças
    }

    pub fn ganhou(&self) -> bool {
        for casa in self.tabuleiro.iter().flatten() {
            if let Casa::Ocupada(peça) = *casa {
                if !self.é_a_vez_de(peça) {
                    // encontrou uma peça do inimigo, logo, o jogo não acabou
                    return false;
                }
            } 
        }
        true
    }

    pub fn empatou(&self) -> bool {
        if self.duas_damas() && self.contador_empate >= 5.0 {
            return true;
        }
        if self.contador_empate >= 20.0 {
            return true;
        }
        false
    }

    fn duas_damas(&self) -> bool {
        let mut damas_branca = 0;
        let mut damas_preta = 0;
        for casa in self.tabuleiro.iter().flatten() {
            if let Casa::Ocupada(peça) = *casa {
                match peça {
                    Pedra::DamaBranca => damas_branca += 1,
                    Pedra::DamaPreta => damas_preta += 1,
                    _ => return false,
                }
                if damas_branca > 1 || damas_preta > 1 { return false; }
            } 
        }
        true
    }

    fn todas_capturas_possiveis(&self) -> Vec<Vec<Jogada>> {
        let mut capturas = vec![];
        let peças = self.peças_da_cor_atual();
        for peça in peças {
            capturas.append(&mut self.calcular_capturas(peça));
        }
        let capturas = capturas.into_iter().filter(|x| !x.is_empty()).collect_vec();
        if capturas.is_empty() { return vec![] }
        let maior_len = capturas.iter().max_by_key(|x| x.len()).unwrap().len();
        capturas.into_iter().filter(|x| x.len() == maior_len).collect()
    }

    fn todos_movimentos_possiveis(&self) -> Vec<Vec<Jogada>> {
        let mut movimentos = vec![];
        let peças = self.peças_da_cor_atual();
        for peça in peças {
            for movimento in self.calcular_movimentos(peça).into_iter() {
                movimentos.push(vec![movimento]);
            }
        }
        movimentos.into_iter().filter(|x| !x.is_empty()).collect()
    }

    pub fn é_a_vez_do_branco(&self) -> bool {
        self.vez == Jogador::Branco
    }

    pub fn get_contador_empate(&self) -> f32 {
        self.contador_empate
    }

    pub fn acabou(&self) -> bool {
        self.ganhou() || self.empatou()
    }

    pub fn imobilizado(&self) -> bool {
        self.jogadas.is_empty()
    }
}

mod test {

}
