extern crate cfg_if;
extern crate wasm_bindgen;

mod ffi;
mod game;
mod utils;

// Re-exports
pub use ffi::*;

// Constants
const SCREEN_SIZE: (u32, u32) = (800, 600);
const START_RADIUS: f32 = 8.0;
const FINAL_RADIUS: f32 = 50.0; // dot will grow from START to FINAL
const SPEED: f32 = 2.0;
