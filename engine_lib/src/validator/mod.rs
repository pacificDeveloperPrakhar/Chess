use crate::evaluator::moves_evaluator;
pub mod epd_to_num;
pub fn is_valid_move(epd: String, move_from: String, move_to: String) -> bool {
    // Split the EPD string into rows
    let epd_rows: Vec<&str> = epd.split('/').collect();

    let mut epd_mod: Vec<Vec<char>> = epd_rows.iter().map(|&row| row.chars().collect()).collect();

    let move_from_chars: Vec<char> = move_from.chars().collect();
    let move_from_row: usize = (move_from_chars[0] as u8 - b'a') as usize;
    let move_from_col: usize = (move_from_chars[1] as u8 - b'1') as usize;

    let move_to_chars: Vec<char> = move_to.chars().collect();
    let move_to_row: usize = (move_to_chars[0] as u8 - b'a') as usize;
    let move_to_col: usize = (move_to_chars[1] as u8 - b'1') as usize;


    let possibilities= match epd_mod[move_from_row][move_from_col] {
        'q' | 'Q' => moves_evaluator::all_valid_queen_moves(epd, move_from),
        'k' | 'K' => moves_evaluator::all_valid_king_moves(epd, move_from),
        'n' | 'N' => moves_evaluator::all_valid_knight_moves(epd, move_from),
        'p' | 'P' => moves_evaluator::all_valid_pawn_moves(epd, move_from),
        'r' | 'R' => moves_evaluator::all_valid_rook_moves(epd, move_from),
        'b' | 'B' => moves_evaluator::all_valid_bishop_moves(epd, move_from),
        _ => String::from("00000000/00000000/00000000/00000000/00000000/00000000/00000000/00000000"),
    };


    println!("{}",&possibilities);
    println!("{}",&move_to);
        let epd_num: u64 = epd_to_num::epd_to_num(possibilities);
    println!("{}",epd_num);
        let move_num: u64 = epd_to_num::move_to_num(move_to);
        if epd_num & move_num > 0{
            return true;
        }
    else{

        return false;
    }
}
