extern crate snake_game;
use std::str::FromStr;

use cucumber::{given, then, when, World, Parameter};
use snake_game::{Game, GameState};
use snake_game::snake::{Snake, Direction};

#[derive(World, Debug, Default, Clone)]
pub struct State {
    input: Option<Game>,
    output: Option<Game>,
}

#[derive(Debug, Default, Parameter)]
#[param(name="key", regex="W|A|S|D")]
enum CuKey {
    #[default]
    W, 
    S, 
    A, 
    D
}
impl FromStr for CuKey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "W" => Self::W,
            "A" => Self::A,
            "S" => Self::S,
            "D" => Self::D,
            invalid => return Err(format!("Invalid key: {invalid}")),
        })
    }
}
impl Into<Direction> for CuKey {
    fn into(self) -> Direction {
        match self {
            CuKey::A => Direction::Left,
            CuKey::D => Direction::Right,
            CuKey::W => Direction::Up,
            CuKey::S => Direction::Down,
        }
    }
}
#[derive(Debug, Default, Parameter, PartialEq)]
#[param(name="dir", regex="up|down|left|right")]
enum CuDirection {
    #[default] 
    Up, 
    Down, 
    Left, 
    Right
}
impl FromStr for CuDirection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "up" => Self::Up,
            "down" => Self::Down,
            "left" => Self::Left,
            "right" => Self::Right,
            invalid => return Err(format!("Invalid direction: {invalid}")),
        })
    }
}
impl PartialEq<Direction> for CuDirection {
    fn eq(&self, other: &Direction) -> bool {
        match *self {
            CuDirection::Up =>
                *other == Direction::Up,
            CuDirection::Down =>
                *other == Direction::Down,
            CuDirection::Left =>
                *other == Direction::Left,
            CuDirection::Right =>
                *other == Direction::Right,
        }
    }
}
impl Into<Direction> for CuDirection {
    fn into(self) -> Direction {
        match self {
            CuDirection::Left => Direction::Left,
            CuDirection::Right => Direction::Right,
            CuDirection::Up => Direction::Up,
            CuDirection::Down => Direction::Down,
        }
    }
}
fn main() {
    futures::executor::block_on(State::run("features/"));
}

//Generic
#[given(expr = "we have a snake game")]
fn given_snake(s: &mut State) {
    let (w, h) = (20,20);
    s.input = Some(Game::new_constructed(
        (w,h),
        Snake::init_snake(3, Direction::Right, (w,h)),
        GameState::Waiting,
        0.0,
        (w-1, h-1) //place it out of the way
        ,0
    ));
}
#[when(expr = "the snake moves to a free spot")]
fn when_free_spot(s: &mut State) {
    let input = s.input.to_owned().unwrap();
    let snake = input.get_snake();
    let mut output = Game::new_constructed(
        input.get_game_size(),
        snake,
        GameState::Moving(Direction::Right),
        0.0,
        input.get_apple_loc(),
        0
    );
    output.update(1.0);
    s.output = Some(output);
}
#[when(expr = "an apple has been eaten")]
fn when_apple_eaten(s: &mut State) {
    let input = s.input.to_owned().unwrap();
    let snake = input.get_snake();
    let (ax, ay) = snake.get_head_pos();
    let mut output = Game::new_constructed(
        input.get_game_size(),
        snake,
        GameState::Moving(Direction::Right),
        0.0,
        (ax+1, ay),
        0
    );
    output.update(2.0); // eat the apple
    s.output = Some(output);
}

#[when(expr = "the snake moves to a spot with an apple")]
fn when_snake_eats_apple(s: &mut State) {
    let mut local_state = s.clone();
    when_apple_eaten(&mut local_state);
    let mut output = local_state.output.to_owned().unwrap();
    output.update(2.0); // run the AteApple logic
    s.output = Some(output);
}

//Snake Growth
#[then(expr = "the snake does not grow")]
fn then_no_grow(s: &mut State) {
    let input_snake_length = s.input.to_owned().unwrap().get_snake().get_length();
    let output_snake_length = s.output.to_owned().unwrap().get_snake().get_length();
    assert_eq!(input_snake_length, output_snake_length, "Snake length changed unexpectedly.");
}
#[then(expr = "the snake grows by one")]
fn then_grow(s: &mut State) {
    let input_snake_length = s.input.to_owned().unwrap().get_snake().get_length();
    let output_snake_length = s.output.to_owned().unwrap().get_snake().get_length();
    assert_ne!(input_snake_length, output_snake_length, "Snake length did not change when expected to.");
    assert!(input_snake_length < output_snake_length, "Snake length did not grow when expected to.");
}

//Snake Death
#[then(expr = "it does not die")]
fn then_no_die(s: &mut State) {
    let output = s.output.to_owned().unwrap();
    assert_ne!(GameState::Dead, output.get_state(), "Snake died unexpectedly");
}

#[when(expr = "the snake moves to a spot that is already occupied by the snake")]
fn when_hit_self(s: &mut State) {
    let input = s.input.to_owned().unwrap();
    let (w,h) = input.get_game_size();
    let mut output = Game::new_constructed(
        (w,h),
        Snake::init_snake(5, Direction::Right, (w,h)),
        input.get_state(),
        0.0,
        input.get_apple_loc() //place it out of the way
        ,0
    );
    output.update_move_dir(Direction::Up);
    output.update(2.0);
    output.update_move_dir(Direction::Left);
    output.update(2.0);
    output.update_move_dir(Direction::Down);
    output.update(2.0);
    s.output = Some(output);
}   
#[then(expr = "it dies")]
fn then_die(s: &mut State) {
    let output = s.output.to_owned().unwrap();
    assert_eq!(GameState::Dead, output.get_state(), "Snake did not die when expected to.");  
}

//Movement
#[when(expr = "the {key} button is pressed")]
fn when_key_press(s: &mut State, key: CuKey) {
    let keypress = match key {
        CuKey::W => piston_window::Key::W,
        CuKey::S => piston_window::Key::S,
        CuKey::A => piston_window::Key::A,
        CuKey::D => piston_window::Key::D,
    };
    let input = s.input.to_owned().unwrap();
    let (w, h) = input.get_game_size();
    let mut output = Game::new_constructed(
        input.get_game_size(),
        Snake::init_snake(1, key.into(), (w,h)),
        GameState::Waiting,
        0.0,
        input.get_apple_loc(),
        0
    );
    output.handle_keypress(keypress);
    output.update(2.0);
    s.output = Some(output);
}

#[then(expr = "the snake moves {dir}")]
fn then_move_direction(s: &mut State, exp_dir: CuDirection) {
    let input_snake_pos = s.input.to_owned().unwrap().get_snake().get_head_pos();
    let output_snake_pos = s.output.to_owned().unwrap().get_snake().get_head_pos();
    match s.output.to_owned().unwrap().get_state() {
        GameState::Moving(dir) => {
            assert_eq!(exp_dir, dir, "Snake did not move in the expected direction.");
            assert_ne!(input_snake_pos, output_snake_pos, "Snake position did not change when expected to.");
        }
        _ => assert!(false, "Snake was not moving"),
    }
} 

#[when(expr = "the snake is moving {dir} and we tap {dir}")]
fn when_reverse_direction(s: &mut State, org_dir: CuDirection, new_dir: CuDirection) {
    let input = s.input.to_owned().unwrap();
    let snake = input.get_snake();
    let mut output = Game::new_constructed(
        input.get_game_size(),
        snake,
        GameState::Waiting,
        0.0,
        input.get_apple_loc(),
        0
    );
    //Move once to the org_dir
    output.update_move_dir(org_dir.into());
    output.update(2.0);
    //Tell the game to move in the opposite direction
    output.update_move_dir(new_dir.into());
    output.update(2.0);
    s.output = Some(output);
}

//Apple Spawning
#[then(expr = "an apple should be spawned")]
fn then_spawn_apple(s: &mut State) {
    let mut output = s.output.to_owned().unwrap();
    let org_apple_loc = output.get_apple_loc();
    assert_eq!(GameState::AteApple, output.get_state(), "GameState did not change to AteApple when expected to.");
    output.update(2.0);
    let new_apple_loc = output.get_apple_loc();
    assert_ne!(org_apple_loc, new_apple_loc, "Apple location was expected to change, but it did not.");
} 

#[when(expr = "the snake moves over the edge")]
fn when_leave_edge(s: &mut State) {
    let input = s.input.to_owned().unwrap();
    let (w, h) = input.get_game_size();
    let mut output = Game::new_constructed(
        input.get_game_size(),
        Snake::new(0, (h/2).abs(), 1, Direction::Left, (w,h)), //pass 0 as x to make snake to appear on left-most edge
        GameState::Waiting,
        0.0,
        input.get_apple_loc(),
        0
    );
    output.update_move_dir(Direction::Left);
    output.update(2.0);
    s.output = Some(output);
}

#[then(expr="the snake appears on the opposite side")]
fn then_appear_opposite_side(s: &mut State) {
    let input = s.input.to_owned().unwrap();
    let output = s.output.to_owned().unwrap();
    let org_pos = input.get_snake().get_head_pos();
    let new_pos = output.get_snake().get_head_pos();
    let game_size = input.get_game_size();
    assert_ne!(org_pos, new_pos, "Position did not change when expected to.");
    assert!(new_pos.0 > 0, "Position went below zero.");
    assert_eq!(new_pos.0, game_size.0 -1, "Position is not wrapping around.");
}

//Points
#[then(expr = "the points go up by one")]
fn then_point(s: &mut State) {
    let org_points = s.input.to_owned().unwrap().get_points();
    let new_points = s.output.to_owned().unwrap().get_points();
    assert_ne!(org_points, new_points, "Points did not change when expected to.");
    assert!(new_points > org_points, "Points did not increase when expected to.");
    assert_eq!(new_points, org_points+1, "Points increased by more than 1.");
}