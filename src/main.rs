extern crate minifb;

use minifb::{Key, Window, WindowOptions};
pub mod pawn;
use pawn::Pawn;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

const COLOR_WHITE_BOARD:u32 = 0xBBBBAA;
const COLOR_BLACK_BOARD:u32 = 0x222222;
const COLOR_MARGIN:u32 = 0xF0F0F0;
const SQUARE_SIZE:usize = WIDTH / 10;
const PIECE_SIZE:usize = WIDTH / 15;

fn draw_chessboard(buffer: &mut Vec<u32>) {
    
    let mut is_white = true;

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
                }
            }
        }
        is_white = !is_white;
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Chessboard",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        draw_chessboard(&mut buffer);
        for i in 1..9{
            let pawn : Pawn = Pawn::new(i,2,true);
            add_piece_on_chessboard(&mut buffer, &pawn);
        }
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
        
    }

    
}

fn add_piece_on_chessboard(buffer: &mut Vec<u32>, pawn:&Pawn){
    for j in (pawn.start_y)*SQUARE_SIZE..(pawn.start_y)*SQUARE_SIZE + PIECE_SIZE {
        for i in (pawn.start_x)*SQUARE_SIZE..(pawn.start_x)*SQUARE_SIZE + PIECE_SIZE {
            let index = j * WIDTH + i;
            buffer[index] = pawn.color;
        }
    }
}
