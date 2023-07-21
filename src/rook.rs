use piece::Piece;
use game::*;
#[derive(Clone)]
pub struct Rook {
    pub coords: [usize;2],
    pub white_side : bool,
}

impl Piece for Rook {
    type Coordinate = [usize;2];

    fn get_name(&self) -> &str {
        "rook"
    }
    fn get_coords(&self) -> &Self::Coordinate {
        &self.coords
    }
    fn set_coords(&mut self, coords: Self::Coordinate) {
        self.coords = coords;
    }
    fn get_white_side(&self) -> bool{
        self.white_side
    }
    fn set_white_side(&mut self, b :bool){
        self.white_side = b;
    }
    fn get_legal_moves(&self) -> Vec<[usize;2]>{
        let mut result : Vec<[usize;2]> = Vec::new();



        return result;
    }
}