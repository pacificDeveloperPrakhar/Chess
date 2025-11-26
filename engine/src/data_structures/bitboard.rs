use std::convert::From;
#[repr(usize)]
pub enum Piece
{
     P=0,
     N=1,
     B=2,
     R=3,
     K=4,
     Q=5,
     A=6,

}
// enum corresponding to the color of the pieces
#[repr(usize)]
pub enum PieceColor
{
    W=0,
    B=1
}

impl From<char> for Piece{
    fn from(c:char)->Self
    {
     match c{
        'Q'=>Piece::Q,
        'K'=>Piece::K,
        'B'=>Piece::B,
        'R'=>Piece::R,
        'P'=>Piece::P,
        'N'=>Piece::N,
         _=> Piece::A
     }
    }
} 