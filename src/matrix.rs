use rand::Rng;
use raylib::prelude::*;

use crate::effects::motion_blur::MotionBlur;

use super::constants::*;
use super::core::*;

struct Character {
    x: i32,
    y: i32,
    char: char,
    ready_to_delete: bool,
}

fn get_random_char() -> char {
    let chars = CHARS.as_bytes();
    let index = rand::rng().random_range(0..chars.len());
    chars[index] as char
}

impl Character {
    fn new() -> Self {
        Self {
            x: rand::rng().random_range(0..WINDOW.width),
            y: -50,
            char: get_random_char(),
            ready_to_delete: false,
        }
    }

    fn update(&mut self) {
        self.y += CONFIG.speed;
        if self.y > WINDOW.height {
            self.ready_to_delete = true;
        }
    }
}

pub struct Matrix {
    characters: Vec<Character>,
    motion_blur: MotionBlur,
}

impl Matrix {
    pub fn init(core: &mut Core) -> Matrix {
        let Core { rl, thread } = core;
        let characters = Vec::new();

        let motion_blur = MotionBlur::init(CONFIG.alpha, rl, &thread);

        Matrix {
            characters,
            motion_blur,
        }
    }
    pub fn game_loop(&mut self, core: &mut Core) {
        let Core { rl, thread } = core;
        self.characters.retain(|char| !char.ready_to_delete);

        let len = self.characters.len();

        if len < CONFIG.limit {
            self.characters.push(Character::new())
        }

        for char in &mut self.characters {
            char.update();
        }

        //Draw

        self.motion_blur.update_texture(rl, &thread, |txt| {
            for char in self.characters.as_slice() {
                txt.draw_text(
                    &char.char.to_string(),
                    char.x,
                    char.y,
                    50,
                    Color::GREEN.brightness(0.3),
                );
            }
        });

        let mut canvas_content = rl.begin_drawing(&thread);
        canvas_content.clear_background(Color::BLACK);
        self.motion_blur.draw_texture(&mut canvas_content);
    }
}
