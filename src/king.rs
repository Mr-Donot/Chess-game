use piece::Piece;

pub struct King {
    pub coords: [usize;2],
    pub white_side : bool,
}

impl Piece for King {
    type Coordinate = [usize;2];

    fn get_name(&self) -> &str {
        "king"
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
}