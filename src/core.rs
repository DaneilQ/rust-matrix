use super::constants::*;

use raylib::prelude::*;

pub struct Core {
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
}

impl Core {
    pub fn init() -> Core {
        let (mut rl, thread) = raylib::init()
            .size(WINDOW.width, WINDOW.height)
            .title(&WINDOW.title)
            .build();

        rl.set_target_fps(WINDOW.fps);
        Core { rl, thread }
    }

    pub fn update<F>(&mut self, mut game_fn: F)
    where
        F: FnMut(&mut Core),
    {
        while !self.rl.window_should_close() {
            game_fn(self);
        }
    }
}
