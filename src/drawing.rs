use image::{DynamicImage, GenericImageView, ImageError, Rgba};
use constants::*;
use pawn::*;
use rook::*;
use knight::*;
use bishop::*;
use queen::*;
use king::*;
use game::*;

use crate::piece::Piece;

pub fn draw_chessboard(buffer: &mut Vec<u32>) -> Vec<u32>{
    let mut is_white = true;
    let mut chessboard: Vec<u32> =  vec![0; WIDTH * HEIGHT];
    for y in 0..10 {
        for x in 0..10 {
        
            let color = if x==0 || y == 0 || x == 9 || y ==  9 {COLOR_MARGIN} else  {if is_white { COLOR_WHITE_BOARD } else { COLOR_BLACK_BOARD }};
            is_white = !is_white;

            let start_x = x * SQUARE_SIZE;
            let start_y = y * SQUARE_SIZE;

            for j in start_y..start_y + SQUARE_SIZE {
                for i in start_x..start_x + SQUARE_SIZE {
                    let index = j * WIDTH + i;
                    buffer[index] = color;
                    chessboard[index] = color;
                }
            }
        }
        is_white = !is_white;
    }
    return chessboard;
}

pub fn load_image(image_path: &str) -> Result<DynamicImage, ImageError> {
    image::open(image_path)
}

pub fn add_every_side_pieces(color_piece: &str)-> Vec<Box<dyn Piece<Coordinate = [usize; 2]>>> {

    let raw_pawn : usize = if color_piece == "black" {2} else {7};
    let raw_pieces : usize = if color_piece == "black" {1} else {8};
    let is_white_side : bool = if color_piece == "black" {false} else {true};
    let mut pieces_container: Vec<Box<dyn Piece<Coordinate = [usize; 2]>>> = Vec::new();
    

    //pawns
    for i in 1..9 {
        let pawn: Box<dyn Piece<Coordinate = [usize; 2]>> = Box::new(Pawn {coords: [i, raw_pawn], white_side: is_white_side});
        pieces_container.push(pawn);
    }
    
    //rooks
    let rook1: Box<dyn Piece<Coordinate = [usize; 2]>> = Box::new( Rook {coords: [1, raw_pieces], white_side: is_white_side});
    let rook2: Box<dyn Piece<Coordinate = [usize; 2]>> = Box::new( Rook {coords: [8, raw_pieces], white_side: is_white_side});
    pieces_container.push(rook1);
    pieces_container.push(rook2);

    //knights
    let knight1: Box<dyn Piece<Coordinate = [usize; 2]>> = Box::new( Knight {coords: [2, raw_pieces], white_side: is_white_side});
    let knight2: Box<dyn Piece<Coordinate = [usize; 2]>> = Box::new( Knight {coords: [7, raw_pieces], white_side: is_white_side});
    pieces_container.push(knight1);
    pieces_container.push(knight2);

    //bishops
    let bishop1: Box<dyn Piece<Coordinate = [usize; 2]>> = Box::new( Bishop {coords: [3, raw_pieces], white_side: is_white_side});
    let bishop2: Box<dyn Piece<Coordinate = [usize; 2]>> = Box::new( Bishop {coords: [6, raw_pieces], white_side: is_white_side});
    pieces_container.push(bishop1);
    pieces_container.push(bishop2);

    let queen: Box<dyn Piece<Coordinate = [usize; 2]>> = Box::new( Queen {coords: [4, raw_pieces], white_side: is_white_side});
    pieces_container.push(queen);

    let king: Box<dyn Piece<Coordinate = [usize; 2]>> = Box::new( King {coords: [5, raw_pieces], white_side: is_white_side});
    pieces_container.push(king);

    return pieces_container;

}

pub fn add_piece_on_chessboard(buffer: &mut Vec<u32>,pawn: &dyn Piece<Coordinate = [usize; 2]>, img_folder: &str, color_piece: &str) {
    
    let image_path = &format!("./img/{}/{}/{}.png", img_folder, color_piece, pawn.get_name())[..];
    let piece_image = load_image(image_path).unwrap();
    
    let mut image_y = 0;
    for j in MARGIN+(pawn.get_coords()[1]) * SQUARE_SIZE..MARGIN+(pawn.get_coords()[1]) * SQUARE_SIZE + PIECE_SIZE { 
        let mut image_x = 0;
        for i in MARGIN+(pawn.get_coords()[0]) * SQUARE_SIZE..MARGIN+(pawn.get_coords()[0]) * SQUARE_SIZE + PIECE_SIZE {
            let index = j * WIDTH + i;
            if image_x < piece_image.width() as usize && image_y < piece_image.height() as usize {
                let rgba = piece_image.get_pixel(image_x as u32, image_y as u32);
                let color = Rgba([rgba[0], rgba[1], rgba[2], rgba[3]]);
                let alpha = color.0[3] as u32;
                let inv_alpha = 255 - alpha;
                if inv_alpha == 0 {
                    buffer[index] = ((color.0[0] as u32) << 16) | ((color.0[1] as u32) << 8) | color.0[2] as u32;
                }
            }
            image_x += 1;
        }
        image_y += 1;
    }

}



pub fn update_board(buffer : &mut Vec<u32>, game : &mut Game, img_folder : &str){
    draw_chessboard(buffer);
    update_board_side(buffer, game, img_folder, "black");
    update_board_side(buffer, game, img_folder, "white");
}

fn update_board_side(buffer : &mut Vec<u32>, game : &mut Game, img_folder : &str, color_piece :&str){
    for i in 1..9{
        for j in 1..9{
            let piece_temp: Option<&mut Box<dyn Piece<Coordinate = [usize; 2]>>> = if color_piece == "white" {game.get_white_piece_by_position(i, j)} else {game.get_black_piece_by_position(i, j)};

            // We can't use pattern matching directly here, so use if let
            if let Some(piece) = piece_temp {
                add_piece_on_chessboard(buffer, piece.as_ref(), img_folder, color_piece);
            }
        }
    }
}

