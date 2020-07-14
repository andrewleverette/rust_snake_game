//! Contains definitions for drawing to the screen

use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(x: i32, y: i32) -> (f64, f64) {
    let scaled_x = (x as f64) * BLOCK_SIZE;
    let scaled_y = (y as f64) * BLOCK_SIZE;

    (scaled_x, scaled_y)
}

pub fn draw_block(x: i32, y: i32, color: Color, ctx: &Context, g: &mut G2d) {
    unimplemented!();
}