use crate::SubPixelPos;
use bevy::prelude::Sprite;

const W: f32 = 160.0;
const H: f32 = 144.0;

pub(crate) struct Scale {
    h: f32, w: f32, pub(crate) g: f32,
    win_w: f32, win_h: f32,
}
// remove when camera issue fixed
impl Default for Scale {
    fn default() -> Self {
        Self {
            h: 0.0, w: 0.0, g: 0.0, win_w: 0.0, win_h: 0.0,
        }
    }
}

impl Scale {
    pub(crate) fn to_screen_coord(&self, coord: f32) -> f32 {
        coord * self.g
    }
    pub(crate) fn to_ig_coord_x(&self, screen_coord_x: f32) -> usize {
        ((screen_coord_x + self.win_w * 0.5) / self.g) as usize
    }
    pub(crate) fn to_ig_coord_y(&self, screen_coord_y: f32) -> usize {
        ((screen_coord_y + self.win_h * 0.5) / self.g) as usize
    }
    pub(crate) fn update(&mut self, window_w: f32, window_h: f32) {
        self.win_w = window_w;
        self.win_h = window_h;
        self.w = window_w / W;
        self.h = window_h / H;
        self.g = if self.h > self.w {
            self.h
        } else {
            self.w
        };
    }
    pub(crate) fn translate_pos(&self, spp: &SubPixelPos, sprite: &Sprite) -> (f32, f32) {
        let mut pos = (spp.x * self.g, spp.y * self.g);
        // align to scale
        pos.0 -= pos.0 % self.g;
        pos.1 -= pos.1 % self.g;
        // align to other sprites
        let w_mod = (sprite.size.x % 2.0) * (self.g / 2.0);
        let h_mod = (sprite.size.y % 2.0) * (self.g / 2.0);
        pos.0 += w_mod;
        pos.1 += h_mod;
        pos
    }
}