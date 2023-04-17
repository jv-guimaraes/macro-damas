#![allow(unused)]
use damas::Partida;
use damas::Jogada;
use damas::Resultado;

use std::io::Write;
use std::io;

fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn input(msg: &str) -> usize {
    print!("{msg} ");
    io::stdout().flush();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("failed to read stdin");
    buffer.trim().parse().unwrap()
}

fn print_lista_de_jogadas(jogadas: Vec<Vec<Jogada>>) {
    for (i, jogada) in jogadas.iter().enumerate() {
        print!("{i}: {:?}   ", jogada);
        if (i+1) % 3 == 0 { println!(); }
    }
    println!();
}

fn main() {
    let mut jogo = Partida::default();

    loop {
        println!("{}", jogo);
        print_lista_de_jogadas(jogo.todas_jogadas_possiveis().to_vec());
        let jogada = input(&format!("Vez do {:?}: ", jogo.get_vez()));
        if let Resultado::FimDoJogo(ganhador) = jogo.jogar(jogada) {
            clear_terminal();
            println!("{}", jogo);
            println!("{:?} ganhou!!!", ganhador);
            std::process::exit(0);
        }
        clear_terminal();
    }
}