use rand::seq::SliceRandom;
use rand::thread_rng;
use std::thread;

use damas::Casa;
use damas::Partida;

pub fn melhor_jogada_preta(partida: &Partida, depth: u32) -> usize {
    if partida.get_vez().é_branco() {
        panic!("Não é a vez do preto!");
    }
    let mut min_value = i32::MAX;
    let mut min_ix = 0;
    for (ix, _) in partida.todas_jogadas_possiveis().iter().enumerate() {
        let mut copia = partida.clone();
        copia.jogar(ix);
        let value = minimax(&copia, depth - 1, true);
        if value < min_value {
            min_value = value;
            min_ix = ix;
        }
    }
    min_ix
}

pub fn melhor_jogada_preta_mt(partida: &Partida, depth: u32) -> usize {
    if partida.get_vez().é_branco() {
        panic!("Não é a vez do preto!");
    }
    let mut handles = Vec::new();
    for (ix, _) in partida.todas_jogadas_possiveis().iter().enumerate() {
        let mut copia = partida.clone();
        copia.jogar(ix);
        handles.push(thread::spawn(move || minimax(&copia, depth - 1, true)));
    }
    let mut values: Vec<_> = handles
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i, v.join().unwrap()))
        .collect();
    values.shuffle(&mut thread_rng());
    values.iter().min_by_key(|x| x.1).unwrap().0
}

pub fn minimax(partida: &Partida, depth: u32, maximizing_player: bool) -> i32 {
    _minimax(partida, depth, i32::MIN, i32::MAX, maximizing_player)
}

fn _minimax(
    partida: &Partida,
    depth: u32,
    mut alpha: i32,
    mut beta: i32,
    maximizing_player: bool,
) -> i32 {
    if depth == 0 || partida.acabou() {
        return evaluate(partida);
    }
    if maximizing_player {
        let mut max_eval = i32::MIN;
        for (ix, _) in partida.todas_jogadas_possiveis().iter().enumerate() {
            let mut copia = partida.clone();
            copia.jogar(ix);
            let eval = _minimax(&copia, depth - 1, alpha, beta, false);
            max_eval = i32::max(eval, max_eval);
            alpha = i32::max(alpha, eval);
            if beta <= alpha {
                break;
            }
        }
        max_eval
    } else {
        let mut min_eval = i32::MAX;
        for (ix, _) in partida.todas_jogadas_possiveis().iter().enumerate() {
            let mut copia = partida.clone();
            copia.jogar(ix);
            let eval = _minimax(&copia, depth - 1, alpha, beta, true);
            min_eval = i32::min(eval, min_eval);
            beta = i32::min(beta, eval);
            if beta <= alpha {
                break;
            }
        }
        min_eval
    }
}

fn evaluate(position: &Partida) -> i32 {
    let mut sum = 0;
    for casa in position.get_tabuleiro().iter().flatten() {
        if let Casa::Ocupada(pedra) = casa {
            if pedra.é_branca() {
                sum += 1;
            } else {
                sum -= 1;
            }
        }
    }
    sum
}
