pub mod constants;
pub mod core;
pub mod effects;
pub mod matrix;
pub mod utils;

use crate::core::*;
use crate::matrix::*;

fn main() {
    let mut engine = Core::init();

    let mut game = Matrix::init(&mut engine);

    engine.update(|core| {
        game.game_loop(core);
    });
}
