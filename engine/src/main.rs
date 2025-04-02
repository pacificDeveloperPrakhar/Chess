mod evaluator;
mod validator;
use evaluator::moves_evaluator;
use validator::epd_to_num;
fn main() {
    println!("Hello, world!");
    let board=validator::all_possible_moves(String::from("qqqqqqqq/pppppppp/00000000/000000P0/00000000/00000000/PPPQPPPP/QQQQQQQQ"),String::from("g4"));
    println!("{:?}",board);
}

