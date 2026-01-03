pub struct Window {
    pub title: &'static str,
    pub width: i32,
    pub height: i32,
    pub fps: u32,
}

pub struct Config {
    pub limit: usize,
    pub speed: i32,
    pub alpha: u8,
}
