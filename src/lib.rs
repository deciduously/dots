extern crate cfg_if;
extern crate wasm_bindgen;

mod ffi;
mod game;
mod util;

// Re-exports
pub use ffi::*;

// Constants
const SCREEN_SIZE: (u16, u16) = (800, 600);
const START_RADIUS: f32 = 8.0;
const FINAL_RADIUS: f32 = 50.0;
const HANG_TIME: u16 = 300;
const SPEED: f32 = 2.0;
const GROWTH_SPEED: f32 = 1.0;
