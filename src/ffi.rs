// FFI.rs contains the public FFI interface for wasm_bindgen
use super::SCREEN_SIZE;
use game::{Level, LevelState};
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
    fn new(l: u8) -> Result<Self, String> {
        Ok(Self {
            level: Level::new(l)?,
            // total tries
        })
    }
}

#[wasm_bindgen]
#[repr(C)]
pub struct Game {
    config: GameConfig,
    current: GameInstance,
    // overall score, levels complete
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

    // Public

    pub fn height(&self) -> u16 {
        self.config.height
    }

    pub fn width(&self) -> u16 {
        self.config.width
    }

    pub fn tick(&mut self) {
        self.current.level.tick().unwrap();
    }

    pub fn handle_click(&mut self, x: f32, y: f32) {
        use self::LevelState::*;
        let state = self.level_state().clone();
        match state {
            Begin => {
                self.current.level.begin().unwrap();
            }
            Waiting => {
                self.add_player(x, y);
            }
            Clicked => {}
            Won => {
                self.next_level();
            }
            Lost => {
                self.restart_level();
            }
        }
    }

    pub fn pack(&self) -> *const f32 {
        self.current.level.pack().unwrap().as_ptr()
    }

    // Private
    fn add_player(&mut self, x: f32, y: f32) {
        self.current.level.add_player(x, y)
    }

    fn next_level(&mut self) {
        let level = self.current.level.level;
        if level < 12 {
            self.current = GameInstance::new(level + 1).unwrap()
        } else {
            // End game!
            self.current = GameInstance::new(level).unwrap();
        }
    }

    fn restart_level(&mut self) {
        let level = self.current.level.level;
        self.current = GameInstance::new(level).unwrap()
    }

    fn level_state(&self) -> LevelState {
        self.current.level.level_state
    }
}
