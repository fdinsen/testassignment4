use piston_window::{text, rectangle, Context, G2d, Glyphs, Transformed};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

fn to_gui_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub(crate) fn to_gui_coord_u32(game_coord: i32) -> u32 {
    to_gui_coord(game_coord) as u32
}

pub(crate) fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_gui_coord(x);
    let gui_y = to_gui_coord(y);

    rectangle(color, [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE], con.transform,g);
}

pub fn draw_text(ctx: &Context,graphics: &mut G2d, glyphs: &mut Glyphs, color: Color, pos: (f64,f64),text: &str,) {
    text::Text::new_color(color, 50)
        .draw(
            text,
            glyphs,
            &ctx.draw_state,
            ctx.transform.trans(pos.0 *BLOCK_SIZE, pos.1*BLOCK_SIZE),
            graphics,
        )
        .unwrap();
}