// FFI.rs contains the public FFI interface for wasm_bindgen
use super::SCREEN_SIZE;
use game::{Level, LevelState};
use std::cell::Cell;
use util::set_panic_hook;
use wasm_bindgen::prelude::*;

// Imports

// using Math.random from JS for colors, positions, directions
#[wasm_bindgen(js_namespace = Math)]
extern "C" {
    pub fn random() -> f32;
// try once you know attempts are working.  you may need ot drop the max level to test restart_game
// #[wasm_bindgen(js_namespace = Date)]
// pub fn now() -> u16;
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
    attempts: Cell<u32>, // these will just be rendered to the dom and updated on level changes?
    current_attempts: Cell<u32>,
    //total_collected: u32, this is a little trickier, come back to it.  the level doesnt report when its over beyond changing the state
    level: Level,
}

impl GameInstance {
    fn new(l: u8) -> Result<Self, String> {
        Ok(Self {
            level: Level::new(l)?,
            attempts: Cell::new(0),
            current_attempts: Cell::new(0),
        })
    }

    fn score(&self) -> [u32; 2] {
        [self.attempts.get(), self.current_attempts.get()]
    }

    fn start_level(&mut self, l: u8) -> Result<(), String> {
        self.level = Level::new(l)?;
        self.current_attempts.set(self.current_attempts.get() + 1);
        Ok(())
    }

    fn next_level(&mut self, l: u8) -> Result<(), String> {
        self.level = Level::new(l + 1)?;
        self.attempts.set(self.current_attempts.get() + 1);
        self.current_attempts.set(0);
        Ok(())
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
        set_panic_hook(); // no_op when compiled with --no-default-features
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
        let state = self.level_state();
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

    pub fn header(&self) -> *const u8 {
        self.current.level.header().unwrap().as_ptr()
    }

    pub fn pack(&self) -> *const f32 {
        self.current.level.pack().unwrap().as_ptr()
    }

    pub fn score(&self) -> *const u32 {
        self.current.score().as_ptr()
    }

    // Private
    fn add_player(&mut self, x: f32, y: f32) {
        self.current.level.add_player(x, y)
    }

    fn next_level(&mut self) {
        let level = self.current.level.level;
        self.current
            .next_level(level)
            .unwrap_or_else(|_| self.restart_game());
    }

    fn restart_game(&mut self) {
        self.current.attempts.set(0);
        self.current.current_attempts.set(0);
        self.current.start_level(1).unwrap();
    }

    fn restart_level(&mut self) {
        let level = self.current.level.level;
        self.current.start_level(level).unwrap();
    }

    fn level_state(&self) -> LevelState {
        self.current.level.level_state
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
