#[derive(Debug, Clone)]
pub struct Pawn{
    pub start_x : usize,
    pub start_y : usize,
    pub white_side : bool,
    pub color:u32
}

impl Pawn{
    pub fn new(start_x:usize, start_y:usize, white_side:bool) -> Pawn{
        Pawn {
            start_x : start_x,
            start_y : start_y,
            white_side : white_side,
            color : if white_side {0xFF0000} else {0x00FF00}
        }
    }
    
}