extern crate piston_window;

mod render;

use piston_window::*;
use piston_window::types::Color;

use render::to_gui_coord_u32;
use snake_game::Game;


const BACK_COLOR: Color = [0.204, 0.286, 0.369, 1.0];

fn main() {
    let (width, height) = (20,20);
    let mut window_settings = WindowSettings::new("Snake", [to_gui_coord_u32(width), to_gui_coord_u32(height)]).exit_on_esc(true);

    //Create window
    let mut window: PistonWindow = window_settings.build().unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {

        if let Some(Button::Keyboard(key)) = event.press_args() {
            //Handle keypress
            game.handle_keypress(key);
        }


        // Draw game
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        // Update the state of the game
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}