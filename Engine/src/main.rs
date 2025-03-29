fn main() {
    println!("Hello, world!");
    let board=all_valid_queen_move(String::from("rnbqkbnr/pppppppp/00000000/0000Q000/00P0P000/00000000/PPPp0PPp/RNBQKBNR"),String::from("d5"));
    println!("{:?}",board);
}

fn all_valid_queen_move(epd: String, move_str: String) -> String {
    // here i have split the string by / then at each row there are characters that correspond to the pieces on the corresponding board square
    let epd: Vec<&str> = epd.split('/').collect();
    let mut epd_mod: Vec<Vec<char>> = epd.iter().map(|&row| row.chars().collect()).collect();
    let mut epd_res: Vec<Vec<char>> = vec![vec!['0'; 8]; 8];;

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
    
    epd_res
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("/")
}