use rand::Rng;
use raylib::prelude::*;

const CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

struct Window {
    title: &'static str,
    width: i32,
    height: i32,
}

struct Config {
    limit: usize,
    speed: i32,
    alpha: u8,
}

const WINDOW: Window = Window {
    title: "MATRIX",
    width: 1920,
    height: 1080,
};

const CONFIG: Config = Config {
    limit: 300,
    speed: 15,
    alpha: 30,
};

fn get_random_char() -> char {
    let chars = CHARS.as_bytes();
    let index = rand::rng().random_range(0..chars.len());
    chars[index] as char
}

struct Character {
    x: i32,
    y: i32,
    char: char,
    ready_to_delete: bool,
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

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW.width, WINDOW.height)
        .title(&WINDOW.title)
        .build();

    rl.set_target_fps(60);

    let mut characters: Vec<Character> = Vec::new();

    let mut texture = rl
        .load_render_texture(&thread, WINDOW.width as u32, WINDOW.height as u32)
        .expect("Couldn't create render texture.");

    while !rl.window_should_close() {
        //State

        characters.retain(|char| !char.ready_to_delete);

        let len = characters.len();

        if len < CONFIG.limit {
            characters.push(Character::new())
        }

        for char in &mut characters {
            char.update();
        }

        //Draw

        {
            let mut texture_content = rl.begin_texture_mode(&thread, &mut texture);
            texture_content.draw_rectangle(
                0,
                0,
                WINDOW.width,
                WINDOW.height,
                Color::new(0, 0, 0, CONFIG.alpha),
            );

            for char in characters.as_slice() {
                texture_content.draw_text(
                    &char.char.to_string(),
                    char.x,
                    char.y,
                    50,
                    Color::GREEN.brightness(0.3),
                );
            }
        }

        let mut canvas_content = rl.begin_drawing(&thread);
        canvas_content.clear_background(Color::BLACK.alpha(1.0));
        canvas_content.draw_texture_pro(
            &texture.texture(),
            Rectangle::new(
                0.0,
                0.0,
                texture.texture.width as f32,
                -texture.texture.height as f32,
            ),
            Rectangle::new(0.0, 0.0, WINDOW.width as f32, WINDOW.height as f32),
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );
    }
}
