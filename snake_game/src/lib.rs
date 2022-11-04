mod render;
pub mod snake;

use render::{draw_block, draw_text, to_gui_coord_u32};
use snake::{Block, Collision, Direction, Snake};

use piston_window::{
    clear, types::Color, Button, Context, G2d, Key, PistonWindow, PressEvent, UpdateEvent,
    WindowSettings, Glyphs,
};
use rand::Rng;
use std::collections::LinkedList;

const APPLE_COLOUR: Color = [0.95, 0.30, 0.1, 1.0];
const SNAKE_COLOUR: Color = [0.18, 0.80, 0.44, 1.0];
const BG_COLOUR: Color = [0.204, 0.286, 0.369, 1.0];
const GAMEOVER_COLOR: Color = [0.91, 0.30, 0.24, 0.5];

const STEP_TIME: f64 = 0.2; // in second
#[derive(Debug, Clone)]
pub struct Game {
    game_size: (i32, i32),

    snake: Snake,
    state: GameState,
    interval: f64,
    apple_loc: (i32, i32),
    points: i32,
}
#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Waiting,
    Moving(Direction),
    AteApple,
    Dead,
}

pub fn play_game(width: i32, height: i32) {
    let window_settings =
        WindowSettings::new("Snake", [to_gui_coord_u32(width), to_gui_coord_u32(height)])
            .resizable(false)    
            .exit_on_esc(true);

    //Create window
    let mut window: PistonWindow = window_settings.build().unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let ref font = assets.join("retro-gaming.ttf");
    let mut glyphs: Glyphs = window.load_font(font).unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            //Handle keypress
            game.handle_keypress(key);
        }

        // Draw game
        window.draw_2d(&event, |ctx, g, _| {
            clear(BG_COLOUR, g);
            game.draw(&ctx, g, &mut glyphs);
        });

        // Update the state of the game
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}

impl Game {
    //Construction
    fn new(width: i32, height: i32) -> Self {
        let snake = Snake::init_snake(3, Direction::Right, (width, height));
        let apple_loc = Game::generate_random_apple_location((width, height), snake.get_body());
        Game {
            game_size: (width, height),
            apple_loc,

            snake,
            state: GameState::Waiting,
            interval: 0.0,
            points: 0,
        }
    }
    pub fn new_constructed(
        game_size: (i32, i32),
        snake: Snake,
        state: GameState,
        interval: f64,
        apple_loc: (i32, i32),
        points: i32,
    ) -> Self {
        Game {
            game_size,
            snake,
            state,
            interval,
            apple_loc,
            points,
        }
    }

    //Game Logic
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
                self.points += 1;
                println!("Points: {:?}", self.points);
                self.snake.grow_snake();
                self.apple_loc =
                    Game::generate_random_apple_location(self.game_size, self.snake.get_body());
                self.state = GameState::Moving(self.snake.prev_dir.clone());
            }
            GameState::Dead => {
                
            }
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
        if self.is_opposite(&dir) {
            return;
        }
        match self.state {
            GameState::Moving(_) | GameState::Waiting => self.state = GameState::Moving(dir),
            _ => return,
        }
    }
    fn generate_random_apple_location(game_size: (i32, i32),snake_body: &LinkedList<Block>,) -> (i32, i32) {
        let mut rng = rand::thread_rng();

        let mut x = rng.gen_range(1..(game_size.0 - 1));
        let mut y = rng.gen_range(1..(game_size.1 - 1));
        //Make sure the apple doesn't intersect with the snake body
        while Snake::intersects_body(&snake_body, (x, y)) {
            x = rng.gen_range(1..(game_size.0 - 1));
            y = rng.gen_range(1..(game_size.1 - 1));
        }
        (x, y)
    }

    //Rendering
    fn draw(&self, ctx: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
        for block in self.snake.get_body() {
            draw_block(SNAKE_COLOUR, block.x, block.y, ctx, g);
        }
        draw_block(APPLE_COLOUR, self.apple_loc.0, self.apple_loc.1, ctx, g);
        if self.state == GameState::Dead{
            draw_text(&ctx, g, glyphs, GAMEOVER_COLOR, (10.0, 10.0), ":C");
        }
        draw_text(&ctx, g, glyphs, GAMEOVER_COLOR, (1.0, 2.0), &self.points.to_string().clone()); 
    }

    //private functions
    fn is_opposite(&self, dir: &Direction) -> bool {
        self.snake.prev_dir.opposite() == *dir
    }
    fn handle_collision(&mut self, col: Collision) {
        match col {
            Collision::Apple => {
                self.state = GameState::AteApple;
            }
            Collision::Snake => self.state = GameState::Dead,
            Collision::None => {}
        }
    }

    //Getters
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
    pub fn get_points(&self) -> i32 {
        self.points
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {}
}
