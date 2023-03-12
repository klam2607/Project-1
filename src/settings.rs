

use rusty_engine::prelude::*;

pub const WINDOW_HEIGHT : f32 = 600.0;
pub const WINDOW_WIDTH : f32 = 800.0;

pub const BACKGROUND_SPRITE_PATH : &str = "sprite/background/background.png";
pub const BACKGROUND_MUSIC_PATH : &str = "bg.mp3";
pub const SFX_PATH : &str = "sfx.mp3";
pub const BACKGROUND_VOLUMN : f32 = 0.2;
pub const SFX_VOLUMN : f32 = 1.0;

pub fn get_window_descriptor() -> WindowDescriptor {
    WindowDescriptor {
        title: "Caro game".to_string(), 
        width: WINDOW_WIDTH, 
        height: WINDOW_HEIGHT,
        resizable: false,
        mode: WindowMode::Windowed,
        ..Default::default()
    }
} 

pub const TITLE_TEXT_X : f32 = 0.0;
pub const TITLE_TEXT_Y : f32 = 275.0;

pub const MAIN_TEXT_LABEL : &str = "main_text";
pub const MAIN_TEXT_X : f32 = 0.0;
pub const MAIN_TEXT_Y : f32 = 235.0;

// Layer related const
pub const BACKGROUND_LAYER : f32 = 0.0;
pub const CELL_LAYER : f32 = 1.0;
pub const SYMBOL_LAYER : f32 = 2.0;
pub const TEXT_LAYER: f32 = 3.0;



