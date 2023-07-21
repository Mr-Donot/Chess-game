use piece::Piece;
use game::*;

#[derive(Clone)]
pub struct Pawn {
    pub coords: [usize;2],
    pub white_side : bool,

}

impl Piece for Pawn {
    type Coordinate = [usize;2];

    fn get_name(&self) -> &str {
        "pawn"
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
        let coords : [usize;2] = [self.get_coords()[0], self.get_coords()[1]];
        let is_white = self.get_white_side();
        if is_white {
            if coords[1] == 7 {
                result.push([coords[0], 6]);
                result.push([coords[0], 5]);
            }
            else{
                if coords[1] > 1 {
                    result.push([coords[0], coords[1]-1]);
                }
            }

        }
        else {
            if coords[1] == 2 {
                result.push([coords[0], 3]);
                result.push([coords[0], 4]);
            }
            else{
                if coords[1] < 8 {
                    result.push([coords[0], coords[1]+1]);
                }
            }
        }




        return result;
    }
}
