use std::collections::LinkedList;

use piston_window::types::Color;
use piston_window::{Context, G2d};

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.0, 0.80, 0.0, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        let mut body = LinkedList::new();

        body.push_back(Block { x: x + 2, y });

        body.push_back(Block { x: x + 1, y });

        body.push_back(Block { x, y });

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(block.x, block.y, SNAKE_COLOR, ctx, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head = self.body.front().unwrap();
        (head.x, head.y)
    }

    pub fn move_forward(&mut self, direction: Option<Direction>) {
        if let Some(dir) = direction {
            self.direction = dir;
        }

        let (x, y) = self.head_position();

        let new_block = match self.direction {
            Direction::Up => Block { x, y: y - 1 },
            Direction::Down => Block { x, y: y + 1 },
            Direction::Left => Block { x: x - 1, y },
            Direction::Right => Block { x: x + 1, y },
        };

        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, direction: Option<Direction>) -> (i32, i32) {
        let (x, y) = self.head_position();

        let moving_direction = match direction {
            Some(dir) => dir,
            None => self.direction,
        };

        match moving_direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }

    pub fn restore_tail(&mut self) {
        if let Some(block) = self.tail.clone() {
            self.body.push_back(block);
        }
    }

    pub fn collision(&self, x: i32, y: i32) -> bool {
        let mut tail = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            tail += 1;
            if tail == self.body.len() - 1 {
                break;
            }
        }
        return false;
    }
}
