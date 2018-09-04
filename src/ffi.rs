// FFI.rs contains the public FFI interface for wasm_bindgen
use super::SCREEN_SIZE;
use game::{Level, PackedDot};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

// Imports

// using Math.random from JS for colors, positions, directions
#[wasm_bindgen(js_namespace = Math)]
extern "C" {
    pub fn random() -> f32;
}

// using Date.now from JS to track state changes
#[wasm_bindgen(js_namespace = Date)]
extern "C" {
    pub fn now() -> u32;
}

// Exports

#[wasm_bindgen]
#[repr(C)]
pub struct GameConfig {
    height: u32,
    width: u32,
}

impl GameConfig {
    fn new() -> Self {
        Self {
            height: SCREEN_SIZE.1,
            width: SCREEN_SIZE.0,
        }
    }
}

#[wasm_bindgen]
#[repr(C)]
pub struct GameInstance {
    level: Level,
}

impl GameInstance {
    fn new(num_dots: u32) -> Self {
        Self {
            level: Level::new(num_dots),
        }
    }
}

#[wasm_bindgen]
#[repr(C)]
pub struct Game {
    config: GameConfig,
    current: GameInstance,
    // overall score, levels completed
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(num_dots: u32) -> Self {
        set_panic_hook();
        Self {
            config: GameConfig::new(),
            current: GameInstance::new(num_dots),
        }
    }

    pub fn load_level(&mut self, num_dots: u32) {
        self.current = GameInstance::new(num_dots);
    }

    pub fn height(&self) -> u32 {
        self.config.height
    }

    pub fn width(&self) -> u32 {
        self.config.width
    }

    pub fn tick(&mut self) {
        self.current.level.tick();
    }

    pub fn get_progress_text(&self) -> String {
        self.current.level.get_progress_text()
    }

    pub fn add_player(&mut self, x: f32, y: f32) {
        self.current.level.add_player(x, y)
    }

    pub fn restart_level(&mut self) {
        self.current.level.restart_level()
    }

    pub fn pack(&self) -> *const PackedDot {
        self.current.level.pack().as_ptr()
    }

    pub fn last_update(&self) -> u32 {
        self.current.level.last_update()
    }

    pub fn num_dots(&self) -> u32 {
        self.current.level.num_dots() as u32
    }
}
