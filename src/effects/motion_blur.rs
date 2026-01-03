use crate::constants::WINDOW;
use raylib::prelude::*;

pub struct MotionBlur {
    alpha: u8,
    texture: RenderTexture2D,
}

impl MotionBlur {
    pub fn init(alpha: u8, rl: &mut RaylibHandle, thread: &RaylibThread) -> MotionBlur {
        let texture = rl
            .load_render_texture(&thread, WINDOW.width as u32, WINDOW.height as u32)
            .expect("Couldn't create render texture.");
        MotionBlur { alpha, texture }
    }

    pub fn update_texture<T>(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, mut lambda: T)
    where
        T: FnMut(&mut RaylibTextureMode<'_, RaylibHandle>),
    {
        //Apply last frame as blur before drawing rest of new texture
        let mut texture_with_blur = rl.begin_texture_mode(&thread, &mut self.texture);

        texture_with_blur.draw_rectangle(
            0,
            0,
            WINDOW.width,
            WINDOW.height,
            Color::new(0, 0, 0, self.alpha),
        );

        //Draw the items inside of the texture
        lambda(&mut texture_with_blur);
    }

    pub fn draw_texture(&mut self, rl: &mut RaylibDrawHandle<'_>) {
        rl.draw_texture_pro(
            &self.texture.texture(),
            Rectangle::new(
                0.0,
                0.0,
                self.texture.texture.width as f32,
                -self.texture.texture.height as f32,
            ),
            Rectangle::new(0.0, 0.0, WINDOW.width as f32, WINDOW.height as f32),
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );
    }
}
