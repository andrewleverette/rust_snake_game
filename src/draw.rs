//! Contains definitions for drawing to the screen

use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(x: i32, y: i32) -> (f64, f64) {
    let scaled_x = (x as f64) * BLOCK_SIZE;
    let scaled_y = (y as f64) * BLOCK_SIZE;

    (scaled_x, scaled_y)
}

pub fn draw_block(x: i32, y: i32, color: Color, ctx: &Context, g: &mut G2d) {
    let (x, y) = to_coord(x, y);

    rectangle(color, [x, y, BLOCK_SIZE, BLOCK_SIZE], ctx.transform, g);
}

pub fn draws_rectangle(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: Color,
    ctx: &Context,
    g: &mut G2d,
) {
    let (x, y) = to_coord(x, y);

    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        ctx.transform,
        g,
    )
}
