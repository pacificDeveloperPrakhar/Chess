use std::cmp;
fn main() {
    println!("Hello, world!");
    let board=all_valid_queen_moves(String::from("rnbqkbnr/pppppppp/00000000/000p0000/00P0P000/00000000/00P00000/rNQqKBNR"),String::from("d8"));
    println!("{:?}",board);
}

fn all_valid_queen_moves(epd: String, move_str: String) -> String {
            // here i have split the string by / then at each row there are characters that correspond to the pieces on the corresponding board square
            let epd: Vec<&str> = epd.split('/').collect();
            let mut epd_mod: Vec<Vec<char>> = epd.iter().map(|&row| row.chars().collect()).collect();
            let mut epd_res: Vec<Vec<char>> = vec![vec!['0'; 8]; 8];
        
            // now i will be modifying the move string which do be somewhat like this a6 or something to more like 1,6
            let move_chars: Vec<char> = move_str.chars().collect();
            let queen_row: usize = (move_chars[0] as u8 - b'a') as usize;
            let queen_col: usize = (move_chars[1] as u8 - b'1') as usize;
        
            let mut row_iter: usize = 0;
            let mut row_start: usize = 0;
            let mut col_iter: usize = 0;
            let mut col_start: usize = 0;
        
            while row_iter < queen_row {
                if epd_mod[row_iter][queen_col] != '0' {
                    row_start = row_iter;
                }
                row_iter += 1;
            }
            // to mark the initial postion for the row wise and mark that epd postion with kill postion or '.' position for the movable position
            let queen_color = epd_mod[queen_row][queen_col].is_lowercase();
            let square_color = epd_mod[row_start][queen_col].is_lowercase();
    
            if epd_mod[row_start][queen_col]=='0'{
                epd_res[row_start][queen_col]='.';
            }
            else if epd_mod[row_start][queen_col]!='0'&&queen_color != square_color {
                epd_res[row_start][queen_col] = '-';
            }
            if row_start<=queen_row&&row_start <= 6 {
                row_start += 1;
            }
        
            while row_start < 8 {
                if epd_mod[row_start][queen_col] == '0' {
                    
                    epd_res[row_start][queen_col] = '.';
                } else {
                    if row_start!=queen_row{
        
                        let queen_color = epd_mod[queen_row][queen_col].is_lowercase();
                        let square_color = epd_mod[row_start][queen_col].is_lowercase();
                        if queen_color != square_color {
                            epd_res[row_start][queen_col] = '-';
                        } 
                        
                        break;
                    }
                    
                }
                row_start += 1;
            }
        
            while col_iter < queen_col {
                if epd_mod[queen_row][col_iter] != '0' {
                    col_start = col_iter;
                }
                col_iter += 1;
            }
        
            // to mark the initial postion for the column wise and mark that epd postion with kill postion or '.' for the movable position
            let queen_color = epd_mod[queen_row][queen_col].is_lowercase();
            let square_color = epd_mod[queen_row][col_start].is_lowercase();
            if epd_mod[queen_row][col_start]=='0'{
                epd_res[queen_row][col_start]='.';
            }
            else if epd_mod[queen_row][col_start]!='0'&&queen_color != square_color {
                epd_res[queen_row][col_start] = '-';
            }
            if col_start<queen_col&&col_start<7{
                col_start+=1;
            }
            while col_start < 8 {
                if epd_mod[queen_row][col_start] == '0' {
                    epd_res[queen_row][col_start] = '.';
                } else {
                        let queen_color = epd_mod[queen_row][queen_col].is_lowercase();
                        let square_color = epd_mod[queen_row][col_start].is_lowercase();
                        if queen_color != square_color {
                            epd_res[queen_row][col_start] = '-';
                        } 
                        if col_iter!=queen_col{
                            break;
                        }
                
                }
                col_start += 1;
            }
  
    // now checking the diagnol postions now-------------------------------------------------------
    //before anything i will be initializing the row_iter and col-iter using the equation of straight line
    // y intercept will be the value of the row_iter and x intercept will be the valie of col_iter 
    row_iter = cmp::max(queen_row as isize - queen_col as isize, 0) as usize;
    row_start = row_iter;
    col_iter = cmp::max(queen_col as isize - queen_row as isize, 0) as usize;
    col_start = col_iter;

    while col_iter < queen_col && row_iter < queen_row {
        if epd_mod[row_iter][col_iter] != '0' {
            row_start = row_iter;
            col_start = col_iter;
        }
        col_iter += 1;
        row_iter += 1;
    }

    println!("{},{},queen-diagonal", col_start, row_start);
    println!("executing {}", epd_mod[row_iter][col_iter]);

    let queen_color = epd_mod[queen_row][queen_col].is_lowercase();
    let square_color = epd_mod[row_start][col_start].is_lowercase();

    if epd_mod[row_start][col_start] == '0' {
        epd_res[row_start][col_start] = '.';
    } else if epd_mod[row_start][col_start] != '0' && queen_color != square_color {
        epd_res[row_start][col_start] = '-';
    }

    if col_start < queen_col && col_start < 7 && row_start < queen_row && row_start < 7 {
        col_start += 1;
        row_start += 1;
    }

    while col_start < 8 && row_start < 8 {
        if epd_mod[row_start][col_start] == '0' {
            epd_res[row_start][col_start] = '.';
        } else {
            if row_start != queen_row && col_start != queen_col {
                let queen_color = epd_mod[queen_row][queen_col].is_lowercase();
                let square_color = epd_mod[row_start][col_start].is_lowercase();
                if queen_color != square_color {
                    epd_res[row_start][col_start] = '-';
                }
                break;
            }
        }
        col_start += 1;
        row_start += 1;
    }

    // Now for the other diagonal '\'
    let y_intercept = queen_row as isize - (7 - queen_col as isize);
    let x_intercept = y_intercept;
    row_iter = cmp::max(y_intercept, 0) as usize;
    row_start = row_iter;
    col_iter = 7 - cmp::max(-1 * x_intercept, 0) as usize;
    col_start = col_iter;

    println!("{},{} queen", queen_col, queen_row);
    println!("{},{} starting", col_iter, row_iter);

    while col_iter > queen_col && row_iter < queen_row {
        if epd_mod[row_iter][col_iter] == '0' {
            epd_res[row_iter][col_iter] = '.';
        } else {
            row_start = row_iter;
            col_start = col_iter;
        }
        col_iter -= 1;
        row_iter += 1;
    }

    let queen_color = epd_mod[queen_row][queen_col].is_lowercase();
    let square_color = epd_mod[row_start][col_start].is_lowercase();

    if queen_color != square_color {
        epd_res[row_start][col_start] = '-';
    }

    if col_start > queen_col && col_start > 0 && row_start < queen_row && row_start < 7 {
        col_start -= 1;
        row_start += 1;
    }

    while col_start > 0 && row_start < 8 {
        println!("{},{} iterating", col_start, row_start);
        if epd_mod[row_start][col_start] == '0' {
            epd_res[row_start][col_start] = '.';
        } else {
            if row_start != queen_row && col_start != queen_col {
                let queen_color = epd_mod[queen_row][queen_col].is_lowercase();
                let square_color = epd_mod[row_start][col_start].is_lowercase();
                if queen_color != square_color {
                    epd_res[row_start][col_start] = '-';
                }
                break;
            }
        }
        col_start -= 1;
        row_start += 1;
    }

    return epd_res
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("/")
}
// bishop valid moves
fn all_valid_bishop_moves(epd: String, move_str: String) -> String {
    let epd: Vec<&str> = epd.split('/').collect();
    let mut epd_mod: Vec<Vec<char>> = epd.iter().map(|&row| row.chars().collect()).collect();
    let mut epd_res: Vec<Vec<char>> = vec![vec!['0'; 8]; 8];

    let move_chars: Vec<char> = move_str.chars().collect();
    let queen_row: usize = (move_chars[0] as u8 - b'a') as usize;
    let queen_col: usize = (move_chars[1] as u8 - b'1') as usize; // Fixed row calculation

    let mut row_iter: usize;
    let mut row_start: usize;
    let mut col_iter: usize;
    let mut col_start: usize;

    row_iter = cmp::max(queen_row as isize - queen_col as isize, 0) as usize;
    row_start = row_iter;
    col_iter = cmp::max(queen_col as isize - queen_row as isize, 0) as usize;
    col_start = col_iter;

    while col_iter < queen_col && row_iter < queen_row {
        if epd_mod[row_iter][col_iter] != '0' {
            row_start = row_iter;
            col_start = col_iter;
        }
        col_iter += 1;
        row_iter += 1;
    }

    println!("{},{},queen-diagonal", col_start, row_start);
    println!("executing {}", epd_mod[row_iter][col_iter]);

    let queen_color = epd_mod[queen_row][queen_col].is_lowercase();
    let square_color = epd_mod[row_start][col_start].is_lowercase();

    if epd_mod[row_start][col_start] == '0' {
        epd_res[row_start][col_start] = '.';
    } else if epd_mod[row_start][col_start] != '0' && queen_color != square_color {
        epd_res[row_start][col_start] = '-';
    }

    if col_start < queen_col && col_start < 7 && row_start < queen_row && row_start < 7 {
        col_start += 1;
        row_start += 1;
    }

    while col_start < 8 && row_start < 8 {
        if epd_mod[row_start][col_start] == '0' {
            epd_res[row_start][col_start] = '.';
        } else {
            if row_start != queen_row && col_start != queen_col {
                let queen_color = epd_mod[queen_row][queen_col].is_lowercase();
                let square_color = epd_mod[row_start][col_start].is_lowercase();
                if queen_color != square_color {
                    epd_res[row_start][col_start] = '-';
                }
                break;
            }
        }
        col_start += 1;
        row_start += 1;
    }

    // Now for the other diagonal '\'
    let y_intercept = queen_row as isize - (7 - queen_col as isize);
    let x_intercept = y_intercept;
    row_iter = cmp::max(y_intercept, 0) as usize;
    row_start = row_iter;
    col_iter = 7 - cmp::max(-1 * x_intercept, 0) as usize;
    col_start = col_iter;

    println!("{},{} queen", queen_col, queen_row);
    println!("{},{} starting", col_iter, row_iter);

    while col_iter > queen_col && row_iter < queen_row {
        if epd_mod[row_iter][col_iter] == '0' {
            epd_res[row_iter][col_iter] = '.';
        } else {
            row_start = row_iter;
            col_start = col_iter;
        }
        col_iter -= 1;
        row_iter += 1;
    }

    let queen_color = epd_mod[queen_row][queen_col].is_lowercase();
    let square_color = epd_mod[row_start][col_start].is_lowercase();

    if queen_color != square_color {
        epd_res[row_start][col_start] = '-';
    }

    if col_start > queen_col && col_start > 0 && row_start < queen_row && row_start < 7 {
        col_start -= 1;
        row_start += 1;
    }

    while col_start > 0 && row_start < 8 {
        println!("{},{} iterating", col_start, row_start);
        if epd_mod[row_start][col_start] == '0' {
            epd_res[row_start][col_start] = '.';
        } else {
            if row_start != queen_row && col_start != queen_col {
                let queen_color = epd_mod[queen_row][queen_col].is_lowercase();
                let square_color = epd_mod[row_start][col_start].is_lowercase();
                if queen_color != square_color {
                    epd_res[row_start][col_start] = '-';
                }
                break;
            }
        }
        col_start -= 1;
        row_start += 1;
    }

    epd_res
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("/")
}

fn all_valid_knight_moves(epd: String, move_str: String) -> String {
    // here i have split the string by / then at each row there are characters that correspond to the pieces on the corresponding board square
    let epd: Vec<&str> = epd.split('/').collect();
    let mut epd_mod: Vec<Vec<char>> = epd.iter().map(|&row| row.chars().collect()).collect();
    let mut epd_res: Vec<Vec<char>> = vec![vec!['0'; 8]; 8];

    // now i will be modifying the move string which do be somewhat like this a6 or something to more like 1,6
    let move_chars: Vec<char> = move_str.chars().collect();
    let knight_row: usize = (move_chars[0] as u8 - b'a') as usize;
    let knight_col: usize = (move_chars[1] as u8 - b'1') as usize;

    let mut row_iter: usize = 0;
    let mut row_start: usize = 0;
    let mut col_iter: usize = 0;
    let mut col_start: usize = 0;

    // knight movable position determination is L shaped i have tried constructing its path using two arrays
    const MOVE2: [i8; 2] = [2, -2];
    const MOVE1: [i8; 2] = [1, -1];

    for &i in &MOVE1 {
        for &j in &MOVE2 {
            let square_col = knight_col as i8 + i;

            let square_row = knight_row as i8 + j;
        

            
                if square_col < 0 || square_col> 7 {
                    continue;
                }
                if square_row< 0 || square_row> 7 {
                    continue;
                }

                let knight_color = epd_mod[knight_row][knight_col].is_lowercase();
                let piece_color = epd_mod[square_row as usize][square_col as usize].is_lowercase();

                if epd_mod[square_row as usize][square_col as usize] != '0' {
                    if knight_color != piece_color {
                        epd_res[square_row as usize][square_col as usize] = '-';
                    }
                } else {
                    epd_res[square_row as usize][square_col as usize] = '.';
                }
            }
        }
    
        for &j in &MOVE1 {
            for &i in &MOVE2 {
                let square_col = knight_col as i8 + i;
    
                let square_row = knight_row as i8 + j;
            
    
                
                    if square_col < 0 || square_col> 7 {
                        continue;
                    }
                    if square_row< 0 || square_row> 7 {
                        continue;
                    }
    
                    let knight_color = epd_mod[knight_row][knight_col].is_lowercase();
                    let piece_color = epd_mod[square_row as usize][square_col as usize].is_lowercase();
    
                    if epd_mod[square_row as usize][square_col as usize] != '0' {
                        if knight_color != piece_color {
                            epd_res[square_row as usize][square_col as usize] = '-';
                        }
                    } else {
                        epd_res[square_row as usize][square_col as usize] = '.';
                    }
                }
            }
    return epd_res
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("/");
}

fn all_valid_pawn_moves(epd: String, move_str: String) -> String {
    let epd: Vec<&str> = epd.split('/').collect();
    let mut epd_mod: Vec<Vec<char>> = epd.iter().map(|&row| row.chars().collect()).collect();
    let mut epd_res: Vec<Vec<char>> = vec![vec!['0'; 8]; 8];

    // Convert "a6" notation to board indices
    let move_chars: Vec<char> = move_str.chars().collect();
    let pawn_col: usize = (move_chars[1] as u8 - b'1') as usize;
    let pawn_row: usize = (move_chars[0] as u8 - b'a') as usize;
    println!("{},{}",pawn_row,pawn_col);
    let pawn_color = epd_mod[pawn_row][pawn_col].is_lowercase();
    
    match pawn_color {
        true => {
            let stepper = if pawn_row == 1 { 2 } else { 1 };
            let mut row_iter = pawn_row + 1;

            while row_iter <= pawn_row + stepper && row_iter <= 7 {
                if epd_mod[row_iter][pawn_col] == '0' {
                    epd_res[row_iter][pawn_col] = '.';
                } else {
                    break;
                }
                row_iter += 1;
            }

            let side_moves: [i8; 2] = [-1, 1];
            for &i in &side_moves {
                let new_col = (pawn_col as isize + i as isize) as usize;
                if new_col < 8 && pawn_row + 1 < 8 && epd_mod[pawn_row + 1][new_col] != '0' {
                    let square_color = epd_mod[pawn_row + 1][new_col].is_lowercase();
                    if !square_color {
                        epd_res[pawn_row + 1][new_col] = '-';
                    }
                }
            }
        }
        false => {
            let stepper = if pawn_row == 6 { -2 } else { -1 };
            let mut row_iter = pawn_row as isize - 1;

            while row_iter >= (pawn_row as isize + stepper) && row_iter >= 0 {
                if epd_mod[row_iter as usize][pawn_col] == '0' {
                    epd_res[row_iter as usize][pawn_col] = '.';
                } else {
                    break;
                }
                row_iter -= 1;
            }

            let side_moves: [i8; 2] = [-1, 1];
            for &i in &side_moves {
                let new_col = (pawn_col as isize + i as isize) as usize;
                if new_col < 8 && pawn_row > 0 && epd_mod[pawn_row - 1][new_col] != '0' {
                    let square_color = epd_mod[pawn_row - 1][new_col].is_lowercase();
                    if square_color {
                        epd_res[pawn_row - 1][new_col] = '-';
                    }
                }
            }
        }
    }

    epd_res
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("/")
}

fn all_valid_king_moves(epd: String, move_str: String) -> String {
    let epd: Vec<&str> = epd.split('/').collect();
    let mut epd_mod: Vec<Vec<char>> = epd.iter().map(|&row| row.chars().collect()).collect();
    let mut epd_res: Vec<Vec<char>> = vec![vec!['0'; 8]; 8];

    let move_chars: Vec<char> = move_str.chars().collect();
    let king_row: usize = (move_chars[0] as u8 - b'a') as usize;
    let king_col: usize=(move_chars[1] as u8 - b'1') as usize;
    println!("{},{}", king_row, king_col);

    let king_color = epd_mod[king_row][king_col].is_lowercase();
    let moves: [i8; 3] = [0, -1, 1]; 

    for &i in &moves {
        for &j in &moves {
            if (king_row as isize + i as isize) < 0 || (king_row as isize + i as isize) > 7 {
                continue;
            } else if (king_col as isize + j as isize) < 0 || (king_col as isize + j as isize) > 7 {
                continue;
            }

            let new_row = (king_row as isize + i as isize) as usize;
            let new_col = (king_col as isize + j as isize) as usize;

            if epd_mod[new_row][new_col] == '0' {
                epd_res[new_row][new_col] = '.';
            } else if epd_mod[new_row][new_col].is_lowercase() != king_color {
                epd_res[new_row][new_col] = '-';
            }
        }
    }

    epd_res
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("/")
}

fn all_valid_rook_moves(epd:String,move_str:String)->String{
        // here i have split the string by / then at each row there are characters that correspond to the pieces on the corresponding board square
        let epd: Vec<&str> = epd.split('/').collect();
        let mut epd_mod: Vec<Vec<char>> = epd.iter().map(|&row| row.chars().collect()).collect();
        let mut epd_res: Vec<Vec<char>> = vec![vec!['0'; 8]; 8];
    
        // now i will be modifying the move string which do be somewhat like this a6 or something to more like 1,6
        let move_chars: Vec<char> = move_str.chars().collect();
        let queen_row: usize = (move_chars[0] as u8 - b'a') as usize;
        let queen_col: usize = (move_chars[1] as u8 - b'1') as usize;
    
        let mut row_iter: usize = 0;
        let mut row_start: usize = 0;
        let mut col_iter: usize = 0;
        let mut col_start: usize = 0;
    
        while row_iter < queen_row {
            if epd_mod[row_iter][queen_col] != '0' {
                row_start = row_iter;
            }
            row_iter += 1;
        }
        // to mark the initial postion for the row wise and mark that epd postion with kill postion or '.' position for the movable position
        let queen_color = epd_mod[queen_row][queen_col].is_lowercase();
        let square_color = epd_mod[row_start][queen_col].is_lowercase();

        if epd_mod[row_start][queen_col]=='0'{
            epd_res[row_start][queen_col]='.';
        }
        else if epd_mod[row_start][queen_col]!='0'&&queen_color != square_color {
            epd_res[row_start][queen_col] = '-';
        }
        if row_start<=queen_row&&row_start <= 6 {
            row_start += 1;
        }
    
        while row_start < 8 {
            if epd_mod[row_start][queen_col] == '0' {
                
                epd_res[row_start][queen_col] = '.';
            } else {
                if row_start!=queen_row{
    
                    let queen_color = epd_mod[queen_row][queen_col].is_lowercase();
                    let square_color = epd_mod[row_start][queen_col].is_lowercase();
                    if queen_color != square_color {
                        epd_res[row_start][queen_col] = '-';
                    } 
                    
                    break;
                }
                
            }
            row_start += 1;
        }
    
        while col_iter < queen_col {
            if epd_mod[queen_row][col_iter] != '0' {
                col_start = col_iter;
            }
            col_iter += 1;
        }
    
        // to mark the initial postion for the column wise and mark that epd postion with kill postion or '.' for the movable position
        let queen_color = epd_mod[queen_row][queen_col].is_lowercase();
        let square_color = epd_mod[queen_row][col_start].is_lowercase();
        if epd_mod[queen_row][col_start]=='0'{
            epd_res[queen_row][col_start]='.';
        }
        else if epd_mod[queen_row][col_start]!='0'&&queen_color != square_color {
            epd_res[queen_row][col_start] = '-';
        }
        if col_start<queen_col&&col_start<7{
            col_start+=1;
        }
        while col_start < 8 {
            if epd_mod[queen_row][col_start] == '0' {
                epd_res[queen_row][col_start] = '.';
            } else {
                    let queen_color = epd_mod[queen_row][queen_col].is_lowercase();
                    let square_color = epd_mod[queen_row][col_start].is_lowercase();
                    if queen_color != square_color {
                        epd_res[queen_row][col_start] = '-';
                    } 
                    if col_iter!=queen_col{
                        break;
                    }
            
            }
            col_start += 1;
        }
    return epd_res
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("/")
}