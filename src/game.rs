use drawing::*;
use piece::*;

pub struct Game {
    pub white_pieces : Vec<Box<dyn Piece<Coordinate = [usize; 2]>>>,
    pub black_pieces : Vec<Box<dyn Piece<Coordinate = [usize; 2]>>>,
}

impl Game {
    
    pub fn init_pieces(&mut self){
        self.white_pieces = add_every_side_pieces("white");
        self.black_pieces = add_every_side_pieces("black");
    }


    pub fn get_white_pieces(&mut self) -> &mut Vec<Box<dyn Piece<Coordinate = [usize; 2]>>>{
        &mut self.white_pieces
    }
    pub fn get_black_pieces(&mut self) -> &mut Vec<Box<dyn Piece<Coordinate = [usize; 2]>>>{
        &mut self.black_pieces
    }

    pub fn get_white_piece_by_position(&mut self, x : usize, y : usize) -> Option< &mut Box<dyn Piece<Coordinate = [usize; 2]>>>{
        for w in self.get_white_pieces(){
            if w.get_coords()[0] == x && w.get_coords()[1] == y{
                return Some(w);
            }
        }
        return None;
    }
    pub fn get_black_piece_by_position(&mut self, x : usize, y : usize) -> Option <&mut Box<dyn Piece<Coordinate = [usize; 2]>>>{
        for b in self.get_black_pieces(){
            if b.get_coords()[0] == x && b.get_coords()[1] == y{
                return Some(b);
            }
        }
        return None;
    }

    pub fn get_piece_by_position(&mut self, x : usize, y : usize)-> Option <&mut Box<dyn Piece<Coordinate = [usize; 2]>>>{
        for b in self.black_pieces.iter_mut().chain(self.white_pieces.iter_mut()){
            if b.get_coords()[0] == x && b.get_coords()[1] == y{
                return Some(b);
            }
        }
        return None;
    }

    pub fn remove_piece(&mut self, x :usize, y :usize){
        self.black_pieces.retain(|p| p.get_coords().as_ref() != [x, y]);
        self.white_pieces.retain(|p| p.get_coords().as_ref() != [x, y]);

    }
    
}