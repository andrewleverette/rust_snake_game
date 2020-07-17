use piston_window::types::Color;
use piston_window::*;

mod draw;
mod game;
mod snake;

use draw::to_coord_u32;
use game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (20, 20);
    let (scaled_x, scaled_y) = to_coord_u32(width, height);

    let mut window: PistonWindow = WindowSettings::new("Snake", [scaled_x, scaled_y])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |ctx, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&ctx, g);
        });

        event.update(|arg| game.update(arg.dt));
    }
}
