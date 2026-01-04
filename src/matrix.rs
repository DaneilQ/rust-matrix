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
    size: i32,
    brightness: f32,
}

impl Character {
    fn new() -> Self {
        let size = rand::rng().random_range(CONFIG.min_font_size..CONFIG.max_font_size);
        Self {
            x: rand::rng().random_range(0..WINDOW.width),
            y: -50,
            char: Character::get_random_char(),
            ready_to_delete: false,
            size,
            brightness: rand::rng().random_range(0.0..0.5),
        }
    }

    fn update(&mut self) {
        self.y += CONFIG.speed * (self.size / 3);
        if self.y > WINDOW.height {
            self.ready_to_delete = true;
        }
    }

    pub fn draw(&self, drawable_rl: &mut RaylibTextureMode<'_, RaylibHandle>) {
        drawable_rl.draw_text(
            &self.char.to_string(),
            self.x,
            self.y,
            self.size,
            Color::GREEN.brightness(self.brightness),
        );
    }

    fn get_random_char() -> char {
        let chars = CHARS.as_bytes();
        let index = rand::rng().random_range(0..chars.len());
        chars[index] as char
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
                char.draw(txt);
            }
        });

        let mut canvas_content = rl.begin_drawing(&thread);
        canvas_content.clear_background(Color::BLACK);
        self.motion_blur.draw_texture(&mut canvas_content);
    }
}
