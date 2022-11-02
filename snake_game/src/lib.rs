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
#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Waiting,
    Moving(Direction),
    AteApple,
    Dead,
}
#[derive(Debug, Clone)]
pub struct Snake {
    body: LinkedList<Block>,
    prev_dir: Direction,
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
pub struct Block {
    x: i32,
    y: i32,
}
impl PartialEq for Block {
    fn eq(&self, other: &Block) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl PartialEq<(i32, i32)> for Block {
    fn eq(&self, other: &(i32, i32)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

enum Collision {
    None, Apple, Snake 
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        let snake = Snake::init_snake(width, height, 3, Direction::Right);
        Game {
            game_size: (width,height),
            apple_loc: Game::generate_random_apple_location((width, height), &snake.body),

            snake,
            state: GameState::Waiting,
            interval: 0.0,

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
                    self.snake.prev_dir = dir.clone();
                    let col = self.snake.check_collision(self.apple_loc);
                    self.handle_collision(col);
                    self.interval = STEP_TIME;
                }
            }
            GameState::AteApple => {
                self.apple_loc =  Game::generate_random_apple_location(self.game_size, &self.snake.body);
                self.state = GameState::Moving(self.snake.prev_dir.clone());
            },
            GameState::Dead => todo!("Dead not implemented"),
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
        match self.state {
            GameState::Moving(_) | GameState::Waiting => self.state = GameState::Moving(dir),
            _ => return,
        }
    }
    pub fn generate_random_apple_location(game_size: (i32, i32), snake_body: &LinkedList<Block>) -> (i32, i32) {
        let mut rng = rand::thread_rng();

        let mut x = rng.gen_range(1..(game_size.0 -1));
        let mut y = rng.gen_range(1..(game_size.1 -1));
        //Make sure the apple doesn't intersect with the snake body
        while Snake::intersects_body(&snake_body, (x,y)) {
            x = rng.gen_range(1..(game_size.0 -1));
            y = rng.gen_range(1..(game_size.1 -1));
        }
        (x, y)
    }
    fn is_opposite(&self, dir: &Direction) -> bool {
        self.snake.prev_dir.opposite() == *dir
    }
    fn handle_collision(&mut self, col: Collision) {
        match col {
            Collision::Apple => {
                self.snake.grow_snake();
                self.state = GameState::AteApple;
            },
            Collision::Snake => self.state = GameState::Dead,
            Collision::None => {},
        }
    }
}

impl Snake {
    fn new(x: i32, y: i32, size: i32, default_move_dir: Direction) -> Self {
        let mut body = LinkedList::new();
        for i in 0..size {
            body.push_back(Block {x: x-i, y });
        }
        Snake { body, prev_dir: default_move_dir }
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
        let (head_x, head_y) = self.get_head_pos();
        self.body.pop_back();
        self.body.push_front(Block {
            x: head_x + x,
            y: head_y + y,
        });
    }
    fn check_collision(&self, apple_loc: (i32, i32)) -> Collision {
        let mut tmp = self.body.clone();
        tmp.pop_front();
        if Snake::intersects_body(&tmp, self.get_head_pos()) {
            return Collision::Snake;
        }
        if self.get_head_pos() == apple_loc {
            return Collision::Apple;
        }
        Collision::None
    }
    fn grow_snake(&mut self) {
        let (x,y) = self.get_tail_pos();
        self.body.push_back(Block {x,y});
    }
    pub fn get_head_pos(&self) -> (i32, i32) {
        match self.body.front() {
            Some(head) => (head.x, head.y),
            None => panic!("Error: Snake has no head!"),
        }
    }
    fn get_tail_pos(&self) -> (i32, i32) {
        match self.body.back() {
            Some(tail) => (tail.x, tail.y),
            None => panic!("Error: Snake has no tail!"),
        }
    }
    pub fn get_length(&self) -> usize {
        self.body.len()
    }
    pub fn init_snake(width :i32, height: i32, size:i32, default_move_dir: Direction) -> Snake {
        Snake::new((width/2).abs(), (height/2).abs(), size, default_move_dir)
    }

    fn intersects_body(body: &LinkedList<Block>, other: (i32, i32)) -> bool {
        for body_part in body {
            if *body_part == other {
                return true;
            }
        }
        return false;
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
