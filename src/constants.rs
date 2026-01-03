use super::utils::*;

pub const CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub const WINDOW: Window = Window {
    title: "MATRIX",
    width: 1920,
    height: 1080,
    fps: 60,
};

//This probably should go inside of a /matrix folder and utilize it locally for its config
pub const CONFIG: Config = Config {
    limit: 500,
    speed: 2,
    alpha: 30,
    min_font_size: 20,
    max_font_size: 50,
};
