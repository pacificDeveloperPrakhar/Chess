mod evaluator;
mod validator;
mod data_structures;
mod bitboard_manipulation;
use bitboard_manipulation::operations::display_bitboard;
use evaluator::moves_evaluator;
use validator::epd_to_num;
use data_structures::*;

fn main() {
    println!("Hello, world!");
    let board=validator::all_possible_moves(String::from("rnbqkbnr/pppppppp/00000000/0000q000/00000000/00000000/PPPPPPPP/RNBQKBNR"),String::from("d5"));
    println!("{:?}",board);
    display_bitboard(1 as u64);
}

