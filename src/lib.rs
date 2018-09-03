extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;
use wasm_bindgen::prelude::*;

const SCREEN_SIZE: (u32, u32) = (800, 600);
const START_RADIUS: f32 = 8.0;
const FINAL_RADIUS: f32 = 50.0; // dot will grow from START to FINAL
const SPEED: f32 = 2.0;

// using Math.random from JS for colors, positions, directions
#[wasm_bindgen(js_namespace = Math)]
extern "C" {
    fn random() -> f32;
}

// using Date.now from JS to track state changes
#[wasm_bindgen(js_namespace = Date)]
extern "C" {
    fn now() -> u32;
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

    // random X,Y that fits in SCREEN_SIZE
    fn random_point() -> Self {
        (
            random() * (SCREEN_SIZE.0 as f32),
            random() * (SCREEN_SIZE.1 as f32),
        )
            .into()
    }

    // Random direction per frame, between SPEED and -SPEED in both axes
    // TODO make x+y always equal SPEED
    fn random_direction() -> Self {
        let twice_speed = SPEED * 2.0;
        (
            (random() * twice_speed) - SPEED,
            (random() * twice_speed) - SPEED,
        )
            .into()
    }

    // TODO fix this - dots wrap sometimes and disappear sometimes
    fn translate(&mut self, td: Point) {
        self.x = (self.x + td.x) % (SCREEN_SIZE.0 as f32);
        self.y = (self.y + td.y) % (SCREEN_SIZE.1 as f32);
    }

    // just a lil Pythagoras, there may be a better way
    // but I haven't done much post-HS math :(
    fn distance(self, target: Point) -> f32 {
        let a = self.x - target.x;
        let b = self.y - target.y;
        (a.powf(2.0) + b.powf(2.0)).sqrt()
    }
}

impl From<(f32, f32)> for Point {
    fn from(pos: (f32, f32)) -> Point {
        Point::new(pos.0, pos.1)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum DotState {
    Floating,
    Growing,
    Full(u32),
    Shrinking,
    Dead,
}

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
                    self.state = Full(now());
                } else if self.radius < FINAL_RADIUS {
                    self.radius += 0.5;
                }
            }
            Full(start_time) => {
                if now() - start_time >= 200 {
                    self.state = Shrinking;
                }
            }
            Shrinking => {
                if self.radius == START_RADIUS {
                    self.state = Dead;
                } else if self.radius > START_RADIUS {
                    self.radius -= 0.5;
                }
            }
            Dead => {}
        }
    }

    fn capture(&mut self) {
        // we've collided
        self.state = DotState::Growing;
    }
}

fn init_dots(num_dots: u32) -> Vec<Dot> {
    let mut ret = Vec::with_capacity(num_dots as usize + 1); // add one to make room for the player dot
    for idx in 0..num_dots {
        ret.push(Dot::new(
            idx,
            Point::random_point(),
            Point::random_direction(),
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
    last_update: u32,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        utils::set_panic_hook();
        Game {
            height: SCREEN_SIZE.1,
            width: SCREEN_SIZE.0,
            dots: init_dots(5),
            last_update: now(),
        }
    }

    pub fn tick(&mut self) {
        for d in &mut self.dots {
            d.tick();
        }
        self.handle_collisions();
        self.last_update = now();
    }

    pub fn last_update(&self) -> u32 {
        self.last_update
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    // TODO design a packed linear memory layout
    // X,Y,RADIUS sorta shindig
    //pub fn dots(&self) -> *const Dot {
    //    self.dots.as_ptr()
    //}

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

    pub fn get_dot_color(&self, id: u32) -> String {
        self.dots[id as usize].color.clone()
    }

    pub fn draw_dot(&self, id: u32) -> bool {
        // True unless we're Dead
        !(self.dots[id as usize].state == DotState::Dead)
    }

    pub fn add_player(&mut self, x: f32, y: f32) {
        let idx = self.dots.len() as u32;
        self.dots.push(Dot::new(
            idx,
            (x, y).into(),
            Point::new(0.0, 0.0),
            DotState::Growing,
        ))
    }

    fn capture_dot(&mut self, id: u32) {
        self.dots[id as usize].capture();
    }

    fn handle_collisions(&mut self) {
        // on each Floating dot, check each Growing, Full, or Shrinking dot
        // get the distance between our two Positions
        // if its less than the sum of the respective radii, store the idx
        // Afterwards, capture each flagged dot

        let mut collided_dots = Vec::new();

        self.dots
            .iter()
            .filter(|d| (*d).state == DotState::Floating)
            .for_each(|active| {
                self.dots
                    .iter()
                    .filter(|d| match d.state {
                        DotState::Growing | DotState::Full(_) | DotState::Shrinking => true,
                        _ => false,
                    }).for_each(|target| {
                        let distance = active.pos.distance(target.pos);
                        let radius_sum = active.radius + target.radius;
                        if distance <= radius_sum {
                            collided_dots.push(active.id);
                        }
                    });
            });
        for idx in &collided_dots {
            self.capture_dot(*idx);
        }
    }
}
