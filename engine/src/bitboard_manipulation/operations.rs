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