use piston_window::types::Color;
use piston_window::*;

use crate::{
    draw::{draw_block, draw_rectangle},
    snake::{Direction, Snake},
};

use rand::{thread_rng, Rng};

const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

pub struct Game {
    snake: Snake,
    width: i32,
    height: i32,
    game_over: bool,
    food_exist: bool,
    food_x: i32,
    food_y: i32,
    waiting_time : f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        Game {
            width,
            height,
            food_exist: true,
            game_over: false,
            food_x: 6,
            food_y: 4,
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
        }
    }

    pub fn draw(&self, context: &Context, gfx: &mut G2d) {
        self.snake.draw(context, gfx);

        if self.food_exist {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, context, gfx);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, context, gfx);
        draw_rectangle(
            BORDER_COLOR,
            0,
            self.height - 1,
            self.width,
            1,
            context,
            gfx,
        );
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, context, gfx);
        draw_rectangle(
            BORDER_COLOR,
            self.width - 1,
            0,
            1,
            self.height,
            context,
            gfx,
        );

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, context, gfx);
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
            _ => Some(self.snake.head_direction()),
        };

        if let Some(dir ) = dir {
            if dir == self.snake.head_direction().opposite(){
                return;
            }
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return  false;
        }
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height -1
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            // self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }
}
