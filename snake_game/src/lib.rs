mod render;

use piston_window::types::Color;
use piston_window::Key;
use render::{draw_block, draw_rectange};
use std::collections::LinkedList;

use piston_window::Context;
use piston_window::G2d;

use rand::{Rng};

const APPLE_COLOR: Color = [0.90, 0.49, 0.13, 1.0];
const BORDER_COLOR: Color = [0.741, 0.765, 0.78, 1.0];
const GAMEOVER_COLOR: Color = [0.91, 0.30, 0.24, 0.5];
const SNAKE_COLOR: Color = [0.18, 0.80, 0.44, 1.0];

const STEP_TIME: f64 = 0.2; // in second
#[derive(Debug, Clone)]
pub struct Game {
    game_size: (i32, i32),

    snake: Snake,
    state: GameState,
    interval: f64,
    apple_loc: (i32, i32),
}
#[derive(Debug, Clone)]
pub enum GameState {
    Waiting,
    Moving(Direction),
    AteApple,
    Dead,
}
#[derive(Debug, Clone)]
pub struct Snake {
    body: LinkedList<Block>,
    prev_dir: Option<Direction>,
}
#[derive(Debug, Clone,PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Self::Up => Direction::Down,
            Self::Down => Direction::Up,
            Self::Left => Direction::Right,
            Self::Right => Direction::Left
        }
    }
}
#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        let snake = Snake::init_snake(width, height);
        Game {
            game_size: (width,height),

            snake,
            state: GameState::Waiting,
            interval: 0.0,

            apple_loc: Game::generate_random_apple_location(width, height),
        }
    }
    pub fn new_constructed(game_size: (i32, i32), snake: Snake, state: GameState, interval: f64, apple_loc: (i32, i32)) -> Self {
        Game {
            game_size,
            snake,
            state,
            interval,
            apple_loc,
        }
    }
    pub fn get_game_size(&self) -> (i32, i32) {
        self.game_size.clone()
    }
    pub fn get_snake(&self) -> Snake {
        self.snake.clone()
    }
    pub fn get_state(&self) -> GameState {
        self.state.clone()
    }
    pub fn get_apple_loc(&self) -> (i32, i32) {
        self.apple_loc.clone()
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        for block in &self.snake.body {
            draw_block(SNAKE_COLOR, block.x, block.y, ctx, g);
        }
        draw_block(APPLE_COLOR, self.apple_loc.0, self.apple_loc.1, ctx, g);
    }
    pub fn update(&mut self, delta_time: f64) {
        match &self.state {
            GameState::Waiting => {
                return;
            }
            GameState::Moving(dir) => {
                self.interval -= delta_time;
                if self.interval <= 0.0 {
                    self.snake.move_snake(dir);
                    self.interval = STEP_TIME;
                }
            }
            GameState::AteApple => todo!(),
            GameState::Dead => todo!(),
        }
    }
    pub fn handle_keypress(&mut self, key: Key) {
        let dir = match key {
            Key::W | Key::Up => Direction::Up,
            Key::S | Key::Down => Direction::Down,
            Key::A | Key::Left => Direction::Left,
            Key::D | Key::Right => Direction::Right,
            _ => return,
        };
        self.update_move_dir(dir);
    }

    pub fn update_move_dir(&mut self, dir: Direction) {
        if self.is_opposite(&dir) {return;}
        self.snake.prev_dir = Some(dir.clone());
        match self.state {
            GameState::Moving(_) | GameState::Waiting => self.state = GameState::Moving(dir),
            _ => return,
        }
    }
    pub fn generate_random_apple_location(width: i32, height: i32) -> (i32, i32) {
        let mut rng = rand::thread_rng();

        let /*mut*/ x = rng.gen_range(1..(width -1));
        let /*mut*/ y = rng.gen_range(1..(height -1));
        //check that apple location is not overlapping snake
        (x, y)
    }
    fn is_opposite(&self, dir: &Direction) -> bool {
        match &self.snake.prev_dir {
            Some(prev) => prev.opposite() == *dir,
            None => false,
        }
    }
}

impl Snake {
    fn new(x: i32, y: i32) -> Self {
        let mut body = LinkedList::new();
        body.push_back(Block { x, y });
        body.push_back(Block { x: x - 1, y });
        body.push_back(Block { x: x - 2, y });
        Snake { body, prev_dir: None }
    }
    pub fn move_snake(&mut self, dir: &Direction) {
        match &dir {
            Direction::Up => self.perform_move_snake(0, -1),
            Direction::Down => self.perform_move_snake(0, 1),
            Direction::Left => self.perform_move_snake(-1, 0),
            Direction::Right => self.perform_move_snake(1, 0),
        }
    }
    fn perform_move_snake(&mut self, x: i32, y: i32) {
        self.body.pop_back();
        let (head_x, head_y) = self.get_head_pos();
        self.body.push_front(Block {
            x: head_x + x,
            y: head_y + y,
        });
    }
    pub fn get_head_pos(&self) -> (i32, i32) {
        match self.body.front() {
            Some(head) => (head.x, head.y),
            None => panic!("Error: Snake has no head!"),
        }
    }
    pub fn get_length(&self) -> usize {
        self.body.len()
    }
    pub fn init_snake(width :i32, height: i32) -> Snake {
        Snake::new((width/2).abs(), (height/2).abs())
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
