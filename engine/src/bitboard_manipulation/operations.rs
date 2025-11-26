use std::vec::Vec;
use crate::data_structures::bitboard::*;
pub fn display_bitboard(board:u64)->String
{
    let mut temp=board;
    let mut result=String::new();
    for rank in 0..8
    {
        for file in 0..8
        {
            if (temp&1)==1
            {
                result=String::from("1 ")+&result;
            }
            else
            {
                result=String::from("0 ")+&result; // calls method of the string newly created and then add it to it
            }
            temp=temp>>1;
        }
        result=String::from("\n")+&result;
    }
    println!("{}",result);
    return result;
}

pub fn convert_to_bitboard(epd:String)
{
    let mut bitboards:[[u64;7];2]=[[0;7];2];
    // convert the epd string to the vector
    let epd_vectorized:Vec<&str>=epd.split(" ").collect();
    let board_state:&str=epd_vectorized[0];
    for (i,c) in board_state.chars().enumerate()
    {
        if(c=='/')
        {
            continue;
        }
        //here is the main conversion
        
        //first check if the character is capital or small
        if (c as u8)>65
        {
         bitboards[PieceColor::B as usize][Piece::from(c) as usize]=i
        }
    }
}