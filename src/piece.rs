use game::*;
pub trait Piece {
    type Coordinate;
    fn get_name(&self) -> &str;
    fn get_coords(&self) -> &Self::Coordinate;
    fn set_coords(&mut self, coords: Self::Coordinate);
    fn get_white_side(&self) -> bool;
    fn set_white_side(&mut self, b :bool);
    fn get_legal_moves(&self) -> Vec<[usize;2]>;
}



