extern crate minifb;
extern crate image;

mod piece;
mod pawn;
mod rook;
mod knight;
mod bishop;
mod queen;
mod king;
use minifb::{Key, Window, WindowOptions};
mod constants;
use constants::*;
pub mod drawing;
use drawing::*;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Chessboard", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });



    while window.is_open() && !window.is_key_down(Key::Escape) {
        draw_chessboard(&mut buffer);
        let img_folder = "resizeSet";
        add_every_side_pieces(&mut buffer, &img_folder, "black");
        add_every_side_pieces(&mut buffer, &img_folder, "white");

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

