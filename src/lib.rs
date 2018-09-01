extern crate cfg_if;
//extern crate rand; // Call out to JS instead
extern crate wasm_bindgen;

mod utils;

//use rand::Rng;
//use std::time::{Instant};
use wasm_bindgen::prelude::*;

// So, your first basic strategy is to just proxy everything in the backend.
// JS will call tick() to update everybody
// JS will call render() to get an array of objects to render
// JS will handle rendering said shapes (all circles) to the canvas

const SCREEN_SIZE: (u32, u32) = (800, 600);
const START_RADIUS: f32 = 10.0;
//const FINAL_RADIUS: f32 = 50.0; // dot will grow from START to FINAL
const SPEED: f32 = 1.5;

//const UPDATES_PER_SECOND: f32 = 60.0;
//const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {r: u8, g: u8, b: u8, a: u8}

impl Color {
    fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r, g, b, a,
        }
    }
}

impl From<[u8; 4]> for Color {
    fn from(input: [u8; 4]) -> Color {
        Color::new(input[0], input[1], input[2], input[3])
    }
}

// rgba color
//fn random_color() -> Color {
//    // TODO avoid colors too similar to the background
//    let mut rng = rand::thread_rng();
//    [
//        rng.gen_range::<u8>(0, 255),
//        rng.gen_range::<u8>(0, 255),
//        rng.gen_range::<u8>(0, 255),
//        240,
//    ].into()
//}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    // spits out a rando
    // pass true to use as a translation point
    //fn random(translation: bool) -> Self {
        //let mut rng = rand::thread_rng();
     //   let l = if translation { -SPEED } else { 0.0 };
      //  let h_x = if translation {
      //      SPEED
      //  } else {
      //      SCREEN_SIZE.0 as f32
      //  };
      //  let h_y = if translation {
      //      SPEED
      //  } else {
       //     SCREEN_SIZE.1 as f32
      ////  };
      //  (rng.gen_range::<f32>(l, h_x), rng.gen_range::<f32>(l, h_y)).into()
   // }
    // This all isn't quite right, but I think it's close enough for now.

    //fn translate(&mut self, td: Point) {
    //    self.x = (self.x + td.x) % SCREEN_SIZE.0 as f32;
    //    self.y = (self.y + td.y) % SCREEN_SIZE.1 as f32;
   //}
}

impl From<(f32, f32)> for Point {
    fn from(pos: (f32, f32)) -> Point {
        Point::new(pos.0, pos.1)
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum DotState {
    Floating,
    Growing,
    Full, // TODO figure out how to store timer anyway - struct field?
    Shrinking,
    Dead,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Dot {
    pos: Point,
    radius: f32,
    translation: Point,
    state: DotState,
    color: Color,
}

impl Dot {
    fn new(pos: Point, translation: Point, state: DotState) -> Self {
        Self {
            pos,
            translation,
            state,
            radius: START_RADIUS,
            color: [128,155,108,240].into(),
        }
    }

    fn tick(&mut self) {
        self.radius += 1.0;
    }
}

#[wasm_bindgen]
pub struct Game {
    pub height: u32,
    pub width: u32,
    dots: Vec<Dot>,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game {
            height: 800,
            width: 600,
            dots: vec![Dot::new(Point::new(400.0,400.0), Point::new(0.5, 0.5), DotState::Floating)],
        }
    }

    pub fn tick(&mut self) {
        for d in &mut self.dots {
            d.tick();
        }
    }
}