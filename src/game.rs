use piston_window::types::Color;
use piston_window::*;

use rand::{thread_rng, Rng};

use crate::draw::{draw_block, draws_rectangle};
use crate::snake::{Direction, Snake};

const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const GAMEOVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,
    food_exists: bool,
    food_x: i32,
    food_y: i32,
    width: i32,
    height: i32,
    game_over: bool,
    wait_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            wait_time: 0.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        self.snake.draw(ctx, g);

        if self.food_exists {
            draw_block(self.food_x, self.food_y, FOOD_COLOR, ctx, g);
        }

        draws_rectangle(0, 0, self.width, 1, BORDER_COLOR, ctx, g);
        draws_rectangle(0, self.height - 1, self.width, 1, BORDER_COLOR, ctx, g);
        draws_rectangle(0, 0, 1, self.height, BORDER_COLOR, ctx, g);
        draws_rectangle(self.width - 1, 0, 1, self.height, BORDER_COLOR, ctx, g);

        if self.game_over {
            draws_rectangle(0, 0, self.width, self.height, GAMEOVER_COLOR, ctx, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.wait_time += delta_time;

        if self.game_over {
            if self.wait_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food()
        }

        if self.wait_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    pub fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    pub fn check_alive(&self, dir: Option<Direction>) -> bool {
        let (x, y) = self.snake.next_head(dir);

        if self.snake.collision(x, y) {
            return false;
        }

        x > 0 && y > 0 && x < self.width - 1 && y < self.height - 1
    }

    pub fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut x = rng.gen_range(1, self.width - 1);
        let mut y = rng.gen_range(1, self.height - 1);

        while self.snake.collision(x, y) {
            x = rng.gen_range(1, self.width - 1);
            y = rng.gen_range(1, self.height - 1);
        }

        self.food_x = x;
        self.food_y = y;
        self.food_exists = true;
    }

    pub fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.wait_time = 0.0;
    }

    pub fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.wait_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }
}
