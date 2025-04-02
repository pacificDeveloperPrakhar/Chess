
pub fn epd_to_num(epd: String) -> u64 {
    let epd: Vec<char> = epd.chars().map(|c| c ).collect(); // Convert chars to u8
    let mut num: u64 = 0;

    for i in 0..64 {
        if epd[i] !='0' {
            num += 1 << i; 
        }
    }

    num 
}
pub fn move_to_num(move_str: String) -> u64 {
    let move_chars: Vec<char> = move_str.chars().collect();
    let row = move_chars[0] as u8 - b'a';
    let col = move_chars[1] as u8 - b'1';
    let nth:u64 = (row * 8 + col) as u64; 
    return 1_u64<<nth ;
}