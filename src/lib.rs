extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use std::time::{Duration, Instant};
use wasm_bindgen::prelude::*;

// So, your first basic strategy is to just proxy everything in the backend.
// JS will call tick() to update everybody
// JS will call render() to get an array of objects to render
// JS will handle rendering said shapes (all circles) to the canvas

const SCREEN_SIZE: (u32, u32) = (800, 600);
const START_RADIUS: f32 = 10.0;
const FINAL_RADIUS: f32 = 50.0; // dot will grow from START to FINAL
const SPEED: f32 = 1.5;

// This all should go over to the frontend I think
//const UPDATES_PER_SECOND: f32 = 60.0;
//const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

// using Math.random from JS for colors, positins, directions
#[wasm_bindgen(js_namespace = Math)]
extern "C" {
    fn random() -> f32;
}

// rgb color
fn random_color() -> String {
    // TODO avoid colors too similar to the background
    format!(
        "#{:x?}{:x?}{:x?}",
        (random() * 255.0) as u8,
        (random() * 255.0) as u8,
        (random() * 255.0) as u8,
    )
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn random_point() -> Self {
        (
            random() * (SCREEN_SIZE.0 as f32),
            random() * (SCREEN_SIZE.1 as f32),
        )
            .into()
    }

    //fn random_direction() -> Self {
    // this is supposed to genrate something between -SPEED and SPEED

    //    (rng.gen_range::<f32>(l, h_x), rng.gen_range::<f32>(l, h_y)).into()
    //}
    //This all isn't quite right, but I think it's close enough for now.

    fn translate(&mut self, td: Point) {
        self.x = (self.x + td.x) % SCREEN_SIZE.0 as f32;
        self.y = (self.y + td.y) % SCREEN_SIZE.1 as f32;
    }
}

impl From<(f32, f32)> for Point {
    fn from(pos: (f32, f32)) -> Point {
        Point::new(pos.0, pos.1)
    }
}

#[derive(Clone, Copy)]
pub enum DotState {
    Floating,
    Growing,
    Full(Instant),
    Shrinking,
    Dead,
}

#[derive(Clone)]
pub struct Dot {
    id: u32,
    pos: Point,
    radius: f32,
    translation: Point,
    state: DotState,
    color: String, // hex, e.g. "#00FF00"
}

impl Dot {
    fn new(id: u32, pos: Point, translation: Point, state: DotState) -> Self {
        Self {
            id,
            pos,
            translation,
            state,
            radius: START_RADIUS,
            color: random_color(),
        }
    }

    fn tick(&mut self) {
        use self::DotState::*;
        // TODO bounce off edges
        match self.state {
            Floating => {
                self.pos.translate(self.translation);
            }
            Growing => {
                if self.radius == FINAL_RADIUS {
                    // To keep track of how long it's been full.
                    self.state = Full(Instant::now());
                } else if self.radius < FINAL_RADIUS {
                    self.radius += 0.5;
                }
            }
            Full(start_time) => {
                if Instant::now() - start_time >= Duration::from_millis(250) {
                    self.state = Shrinking;
                }
            }
            Shrinking => {
                if self.radius == START_RADIUS {
                    self.state = Dead;
                } else if self.radius > START_RADIUS {
                    self.radius -= 1.0;
                }
            }
            Dead => {}
        }
    }
}

fn init_dots<'a>(num_dots: u32) -> Vec<Dot> {
    let mut ret = Vec::with_capacity(num_dots as usize + 1); // add one to make room for the player dot
    for idx in 0..num_dots {
        ret.push(Dot::new(
            idx,
            Point::random_point(),
            (0.5, 0.5).into(),
            DotState::Floating,
        ));
    }
    ret
}

#[wasm_bindgen]
pub struct Game {
    height: u32,
    width: u32,
    dots: Vec<Dot>,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game {
            height: SCREEN_SIZE.1,
            width: SCREEN_SIZE.0,
            dots: init_dots(5),
        }
    }

    pub fn tick(&mut self) {
        for d in &mut self.dots {
            d.tick();
        }
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn dots(&self) -> *const Dot {
        self.dots.as_ptr()
    }

    pub fn num_dots(&self) -> usize {
        self.dots.len()
    }

    pub fn get_dot_radius(&self, id: u32) -> f32 {
        self.dots[id as usize].radius
    }

    pub fn get_dot_x(&self, id: u32) -> f32 {
        self.dots[id as usize].pos.x
    }

    pub fn get_dot_y(&self, id: u32) -> f32 {
        self.dots[id as usize].pos.y
    }

    // returns the hex code
    pub fn get_dot_color(&self, id: u32) -> String {
        self.dots[id as usize].color.clone()
    }
}
