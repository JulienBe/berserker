use crate::SubPixelPos;
use bevy::prelude::Sprite;

const W: f32 = 160.0;
const H: f32 = 144.0;

pub(crate) struct Scale { h: f32, w: f32, pub(crate) g: f32 }
// remove when camera issue fixed
impl Default for Scale {
    fn default() -> Self {
        Self {
            h: 0.0, w: 0.0, g: 0.0,
        }
    }
}

impl Scale {
    pub(crate) fn update(&mut self, window_w: f32, window_h: f32) {
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