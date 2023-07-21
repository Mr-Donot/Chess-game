extern crate minifb;
extern crate image;

mod piece;
mod pawn;
mod rook;
mod knight;
mod bishop;
mod queen;
mod king;
mod game;
use minifb::{Key, Window, WindowOptions};
use minifb::{MouseButton, MouseMode};
mod constants;
use constants::*;
mod drawing;
use game::*;
use drawing::*;
use piece::*;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut prev_mouse_down = false;
    let img_folder = "resizeSet";
    let mut game : Game = Game {white_pieces : Vec::new(), black_pieces : Vec::new()};
    let mut window = Window::new("Chessboard", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        
    
    game.init_pieces();
    update_board(&mut buffer, &mut game, img_folder);

    let mut holding_piece : bool = false;
    let mut coords_piece : [usize;2] = [0,0];
    while window.is_open() && !window.is_key_down(Key::Escape) {

        let mouse_down = window.get_mouse_down(MouseButton::Left);
        if mouse_down && !prev_mouse_down {
            // Get mouse coordinates on mouse click
            let mouse_coords = window.get_mouse_pos(MouseMode::Clamp).unwrap_or((0.0, 0.0));

            // Convert to i32 (if needed)
            let x:usize = mouse_coords.0 as usize;
            let y:usize = mouse_coords.1 as usize;

            let pos_x:usize = x / SQUARE_SIZE;
            let pos_y:usize = y / SQUARE_SIZE;                

            if ! holding_piece {
                coords_piece = [pos_x, pos_y];
                let chosen_piece = game.get_piece_by_position(pos_x, pos_y);
                match chosen_piece {
                    Some(_) => {holding_piece = true}
                    None => {}
                }
            }
            else {
                let destination = [pos_x, pos_y];
                if coords_piece != destination {
                    game.remove_piece(destination[0], destination[1]);
                }

                let chosen_piece = game.get_piece_by_position(coords_piece[0], coords_piece[1]);
                match chosen_piece {
                    Some(_) => {
                        let unwrap_piece = chosen_piece.unwrap();
                        if is_legal_move(unwrap_piece, &destination){
                            unwrap_piece.set_coords(destination);
                        }
                    }
                    None => {}
                }   
                
                update_board(&mut buffer, &mut game, img_folder);
                holding_piece = false;
            }
        }
        prev_mouse_down = mouse_down;
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}


fn is_legal_move(piece : &mut Box<dyn Piece<Coordinate = [usize;2]>>, destination : &[usize;2]) -> bool{
    return (if piece.get_legal_moves().contains(destination){true} else {false});

}