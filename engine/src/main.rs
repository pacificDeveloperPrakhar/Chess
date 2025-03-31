mod evaluator;
mod validator;
use evaluator::moves_evaluator;
use validator::epd_to_num;
fn main() {
    println!("Hello, world!");
    let board=validator::is_valid_move(String::from("rnbqkbnr/pppppppp/00000000/000p0000/00P0P000/00000000/00P00000/rNQnKBNR"),String::from("h4"),String::from("h2"));
    println!("{:?}",board);
}

