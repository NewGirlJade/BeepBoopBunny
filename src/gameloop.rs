use crate::state::GameState;

pub fn update(gamestate: &mut GameState) {
    gamestate.input.update()
}

pub fn draw(gamestate: &mut GameState) {}
