//                 //
/* * gameloop.rs * */
//                 //
// contains the update and draw functions called by main.rs

use agb::display::Rgb;
use agb::input::Button;

use crate::state::GameState;

pub fn update(gamestate: &mut GameState) {
    gamestate.input.update();
    gamestate.game_progress += 1;
}
pub fn draw(gamestate: &mut GameState) {
    let mut gfx = gamestate.gba.graphics.get();
    const RED: agb::display::Rgb15 = Rgb::new(255, 0, 0).to_rgb15();
    const GREEN: agb::display::Rgb15 = Rgb::new(0, 255, 0).to_rgb15();
    const BLUE: agb::display::Rgb15 = Rgb::new(0, 0, 255).to_rgb15();
    const BLACK: agb::display::Rgb15 = Rgb::new(0, 0, 0).to_rgb15();

    if gamestate.input.is_pressed(Button::L) {
        gfx.set_background_palette_colour(0, 0, RED);
    }
    if gamestate.input.is_pressed(Button::B) {
        gfx.set_background_palette_colour(0, 0, GREEN);
    }
    if gamestate.input.is_pressed(Button::R) {
        gfx.set_background_palette_colour(0, 0, BLUE);
    }
    if gamestate.input.is_pressed(Button::A) {
        gfx.set_background_palette_colour(0, 0, BLACK);
    }

    gfx.frame().commit();
}
