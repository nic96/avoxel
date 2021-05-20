use bevy::prelude::*;

#[derive(Debug)]
pub struct FogSettings {
    pub fog_color: Color,
    pub fog_near: f32,
    pub fog_far: f32,
}

impl Default for FogSettings {
    fn default() -> Self {
        Self {
            fog_color: Color::rgb_u8(92, 119, 127),
            fog_near: 100.0,
            fog_far: 512.,
        }
    }
}
