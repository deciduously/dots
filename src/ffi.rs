// FFI.rs contains the public FFI interface for wasm_bindgen
use super::SCREEN_SIZE;
use game::Level;
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
    pub fn now() -> u16;
}

// Exports

#[wasm_bindgen]
#[repr(C)]
pub struct GameConfig {
    height: u16,
    width: u16,
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
    fn new(l: u16) -> Result<Self, ::std::io::Error> {
        Ok(Self {
            level: Level::new(l)?,
        })
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
    pub fn new() -> Self {
        set_panic_hook();
        Self {
            config: GameConfig::new(),
            current: GameInstance::new(1).unwrap(),
        }
    }

    pub fn height(&self) -> u16 {
        self.config.height
    }

    pub fn width(&self) -> u16 {
        self.config.width
    }

    pub fn tick(&mut self) {
        self.current.level.tick();
    }

    // TODO make just one handle_click and move all of this back into the wasm
    // dispatch action by context

    pub fn add_player(&mut self, x: f32, y: f32) {
        self.current.level.add_player(x, y)
    }

    pub fn next_level(&mut self) {
        let level = self.current.level.level;
        if level < 7 {
            self.current = GameInstance::new(level + 1).unwrap()
        } else {
            self.restart_level()
        }
    }

    pub fn restart_level(&mut self) {
        self.current.level.restart_level().unwrap()
    }

    pub fn pack(&self) -> *const f32 {
        self.current.level.pack().unwrap().as_ptr()
    }
}
