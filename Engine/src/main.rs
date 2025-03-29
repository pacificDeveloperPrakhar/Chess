use std::cmp;
fn main() {
    println!("Hello, world!");
    let board=all_valid_knight_moves(String::from("rnbqkbnr/pppppppp/00000000/0000n000/00P0P000/00000000/PPPp0PPp/RNBQKBNR"),String::from("d5"));
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
    if queen_color != square_color {
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
    if queen_color != square_color {
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
    row_start=0;
    row_iter=cmp::max(queen_row as i32 -queen_col as i32,0) as usize;
    col_start=0;
    col_iter=cmp::max(queen_col as i32-queen_row as i32,0) as usize;
    while col_iter<queen_col&&row_iter<queen_row{
        if epd_mod[row_iter][col_iter]=='0'{
            epd_res[row_iter][col_iter]='.';
        }
        else {
            row_start=row_iter;
            col_start=col_iter;
        }
        col_iter+=1;
        row_iter+=1;
    }
    let queen_color = epd_mod[queen_row][queen_col].is_lowercase();
    let square_color = epd_mod[row_start][col_start].is_lowercase();
    if queen_color!=square_color{
        epd_res[row_start][col_start]='-';
    }
    if (col_start<queen_col&&col_start<7)&&(row_start<queen_row&&col_start<7){
        col_start+=1;
        row_start+=1;
    }
       
    while col_start<8&&row_start<8{
     if epd_mod[row_start][col_start]=='0'{
        epd_res[row_start][col_start]='.'
     }
     else{
        if row_start!=queen_row&&col_start!=queen_col{
            let queen_color = epd_mod[queen_row][queen_col].is_lowercase();
            let square_color = epd_mod[row_start][col_start].is_lowercase();
            if queen_color!=square_color{
               epd_res[row_start][col_start]='-';
            }
            break;
        }
     }
     col_start+=1;
     row_start+=1;
    }
    // now for the another diagnol attacking position '\' <- this one
    let y_intercept = queen_row - (7 - queen_col);
    let x_intercept = y_intercept;
    row_start = 0;
    row_iter = cmp::max(y_intercept, 0) as usize;
    col_start = 0;
    col_iter = 7-cmp::max(-1* x_intercept as i32, 0) as usize;
    println!("{},{} queen",queen_col,queen_row);
    println!("{},{} starting",col_iter,row_iter);
    while col_iter>queen_col&&row_iter<queen_row{
        if epd_mod[row_iter][col_iter]=='0'{
            epd_res[row_iter][col_iter]='.';
        }
        else {
            row_start=row_iter;
            col_start=col_iter;
        }
        col_iter-=1;
        row_iter+=1;
    }
    let queen_color = epd_mod[queen_row][queen_col].is_lowercase();
    let square_color = epd_mod[row_start][col_start].is_lowercase();
    if queen_color!=square_color{
        epd_res[row_start][col_start]='-';
    }
    if (col_start>queen_col&&col_start>0)&&(row_start<queen_row&&col_start<7){
        col_start-=1;
        row_start+=1;
    }
       
    while col_start>0&&row_start<8{
        println!("{},{} iterating",col_start,row_start);
     if epd_mod[row_start][col_start]=='0'{
        epd_res[row_start][col_start]='.'
     }
     else{
        if row_start!=queen_row&&col_start!=queen_col{
            let queen_color = epd_mod[queen_row][queen_col].is_lowercase();
            let square_color = epd_mod[row_start][col_start].is_lowercase();
            if queen_color!=square_color{
               epd_res[row_start][col_start]='-';
            }
            break;
        }
     }
     col_start-=1;
     row_start+=1;
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
    let queen_col: usize = (move_chars[1] as u8 - b'1') as usize;

    let mut row_iter: usize = 0;
    let mut row_start: usize = 0;
    let mut col_iter: usize = 0;
    let mut col_start: usize = 0;

    row_start = 0;
    row_iter = cmp::max(queen_row as i32 - queen_col as i32, 0) as usize;
    col_start = 0;
    col_iter = cmp::max(queen_col as i32 - queen_row as i32, 0) as usize;

    while col_iter < queen_col && row_iter < queen_row {
        if epd_mod[row_iter][col_iter] == '0' {
            epd_res[row_iter][col_iter] = '.';
        } else {
            row_start = row_iter;
            col_start = col_iter;
        }
        col_iter += 1;
        row_iter += 1;
    }

    let queen_color = epd_mod[queen_row][queen_col].is_lowercase();
    let square_color = epd_mod[row_start][col_start].is_lowercase();
    if queen_color != square_color {
        epd_res[row_start][col_start] = '-';
    }

    if (col_start < queen_col && col_start < 7) && (row_start < queen_row && col_start < 7) {
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

    let y_intercept = queen_row as isize - (7 - queen_col) as isize;
    let x_intercept = y_intercept;
    row_start = 0;
    row_iter = cmp::max(y_intercept, 0) as usize;
    col_start = 0;
    col_iter = 7 - cmp::max(-1 * x_intercept, 0) as usize;

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

    if (col_start > queen_col && col_start > 0) && (row_start < queen_row && col_start < 7) {
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
        .join("/");
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
