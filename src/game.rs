// game.rs contains the game logic
use super::{FINAL_RADIUS, SCREEN_SIZE, SPEED, START_RADIUS};
use ffi::{now, random};
use std::{collections::HashMap, fmt};

// UTILITY

// Create initial dot layout
fn init_dots(l: u8) -> Result<HashMap<u8, Dot>, String> {
    let (total_dots, _) = level(l)?;
    let mut ret = HashMap::new();
    for idx in 0..total_dots {
        ret.insert(
            idx,
            Dot::new(
                Point::random_point(),
                Point::random_direction(),
                DotState::Floating,
            ),
        );
    }
    Ok(ret)
}

// rgb color
fn random_color() -> Color {
    // TODO avoid colors too similar to the background
    (
        (random() * 255.0) as u8,
        (random() * 255.0) as u8,
        (random() * 255.0) as u8,
    )
        .into()
}

#[derive(Clone, Copy, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:x?}{:x?}{:x?}", self.r, self.g, self.b)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(rgb: (u8, u8, u8)) -> Self {
        Self {
            r: rgb.0,
            g: rgb.1,
            b: rgb.2,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    // random X,Y that fits in SCREEN_SIZE and is at least START_RADIUS from an edge
    fn random_point() -> Self {
        let min = START_RADIUS * 2.0 + 0.1;
        let x_bound = SCREEN_SIZE.0 as f32;
        let y_bound = SCREEN_SIZE.1 as f32;
        let mut ret: Self = (
            random() * (x_bound - min) + min,
            random() * (y_bound - min) + min,
        )
            .into();
        if x_bound - ret.x <= min {
            ret.x -= min;
        }
        if y_bound - ret.y <= min {
            ret.y -= min;
        }
        ret
    }

    // Random direction at SPEED to translate each frame
    fn random_direction() -> Self {
        let offset_percent = random();
        let neg_x = random();
        let neg_y = random();
        let mut ret: Self = (SPEED * offset_percent, SPEED * (1.0 - offset_percent)).into();
        if neg_x >= 0.5 {
            ret.x = -ret.x;
        }
        if neg_y >= 0.5 {
            ret.y = -ret.y;
        }
        ret
    }

    fn translate(&mut self, td: Point) {
        // Edge/bounce detection is handled by the containing Dot
        self.x += td.x;
        self.y += td.y
    }

    // Thanks, Pythagoras!
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
    Full(u16),
    Shrinking,
    Dead,
}

// because I'm stubborn and enjoy my Full(u16)
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PackedDotState {
    Floating = 1,
    Growing = 2,
    Full = 3,
    Shrinking = 4,
    Dead = 5,
}

impl From<DotState> for PackedDotState {
    fn from(ds: DotState) -> Self {
        use self::PackedDotState::*;
        match ds {
            DotState::Floating => Floating,
            DotState::Growing => Growing,
            DotState::Full(_) => Full,
            DotState::Shrinking => Shrinking,
            DotState::Dead => Dead,
        }
    }
}

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
            color: random_color(),
        }
    }

    fn tick(&mut self) {
        use self::DotState::*;
        // TODO bounce off edges
        match self.state {
            Floating => {
                self.pos.translate(self.translation);
                self.handle_bounce();
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
                // hang out at full for 300ms
                if now() - start_time >= 300 {
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
        self.state = DotState::Growing;
    }

    fn handle_bounce(&mut self) {
        let top_bottom =
            self.pos.x <= self.radius || self.pos.x >= (SCREEN_SIZE.0 as f32 - self.radius);
        let left_right =
            self.pos.y <= self.radius || self.pos.y >= (SCREEN_SIZE.1 as f32 - self.radius);

        if top_bottom && left_right {
            // we hit a corner
            self.translation = (-self.translation.x, -self.translation.y).into();
        } else if top_bottom {
            // We hit either the top or the bottom
            self.translation = (-self.translation.x, self.translation.y).into();
        } else if left_right {
            // We hit the left or right side
            self.translation = (self.translation.x, -self.translation.y).into();
        }
    }

    fn pack(&self) -> PackedDot {
        let data_vec = vec![
            self.pos.x,
            self.pos.y,
            self.radius,
            f32::from(PackedDotState::from(self.state) as u8),
            f32::from(self.color.r),
            f32::from(self.color.g),
            f32::from(self.color.b),
        ];
        let mut packed: PackedDot = [0.0; 7];
        packed[..7].copy_from_slice(&data_vec[..7]);
        packed
    }
}

// Array layout:
// [f32: 7]  x | y | radius | DotState | r | g | b
pub type PackedDot = [f32; 7];

// this is the first few f32s in the array
// level_number | level_state | total_dots | win_threshold | caputured_dots | last_update
pub type LevelHeader = [f32; 6];

pub struct Level {
    dots: HashMap<u8, Dot>,
    last_update: u16,
    pub level: u8,
    pub level_state: LevelState,
}

impl Level {
    // Public

    pub fn new(l: u8) -> Result<Level, String> {
        Ok(Level {
            dots: HashMap::new(),
            last_update: now(),
            level: l,
            level_state: LevelState::Begin,
        })
    }

    pub fn begin(&mut self) -> Result<(), String> {
        self.dots = init_dots(self.level)?;
        self.level_state = LevelState::Waiting;
        Ok(())
    }

    pub fn tick(&mut self) -> Result<(), String> {
        match self.level_state {
            LevelState::Begin | LevelState::Won | LevelState::Lost => {
                self.last_update = now();
            }
            _ => {
                for d in self.dots.values_mut() {
                    d.tick();
                }
                self.handle_collisions();
                self.handle_death()?;
                self.last_update = now();
            }
        }
        Ok(())
    }

    pub fn add_player(&mut self, x: f32, y: f32) {
        if self.level_state == LevelState::Waiting {
            let idx = self.dots.len() as u8;
            self.dots.insert(
                idx,
                Dot::new((x, y).into(), Point::new(0.0, 0.0), DotState::Growing),
            );
            self.level_state = LevelState::Clicked;
        }
    }

    pub fn pack(&self) -> Result<Vec<f32>, String> {
        let header = self.header()?;
        let num_dots_int = header[1] as i32 as usize;
        let mut ret = Vec::with_capacity(num_dots_int * 7 + header.len());
        for e in header.iter().cloned() {
            ret.push(e)
        }
        for dot in self.dots.values() {
            let packed = dot.pack();
            for i in packed.iter().cloned() {
                ret.push(i)
            }
        }
        Ok(ret)
    }

    // Private

    fn capture_dot(&mut self, id: u8) {
        self.dots
            .entry(id)
            // this code path should never execute
            // in case it does, add a dead, motionless corner Dot
            // which is a bad way to handle this - setting it to Growing nullifies the point
            // but, again, we're never going to hit this fn with an ID from a dot that doesn't exist
            // the ID being passed in is read directly from the Dots hashmap, and we never remove keys
            .or_insert_with(|| Dot::new((0.0, 0.0).into(), (0.0, 0.0).into(), DotState::Dead))
            .capture();
    }

    fn handle_collisions(&mut self) {
        // on each Floating dot, check each Growing, Full, or Shrinking dot
        // get the distance between our two Positions
        // if its less than the sum of the respective radii, store the idx
        // Afterwards, capture each flagged dot

        let mut collided_dots = Vec::new();

        self.dots
            .iter()
            .filter(|(_, d)| (*d).state == DotState::Floating)
            .for_each(|(idx, active)| {
                self.dots
                    .iter()
                    .filter(|(_, d)| match d.state {
                        DotState::Growing | DotState::Full(_) | DotState::Shrinking => true,
                        _ => false,
                    }).for_each(|(_, target)| {
                        let distance = active.pos.distance(target.pos);
                        let radius_sum = active.radius + target.radius;
                        if distance <= radius_sum {
                            collided_dots.push(*idx);
                        }
                    });
            });
        for idx in &collided_dots {
            self.capture_dot(*idx);
        }
    }

    // Check if we're all dead - the only dots are either Dead or Floating
    fn handle_death(&mut self) -> Result<(), String> {
        if self.level_state == LevelState::Clicked {
            let active = self
                .dots
                .values()
                .filter(|d| match d.state {
                    DotState::Growing | DotState::Full(_) | DotState::Shrinking => true,
                    _ => false,
                }).collect::<Vec<&Dot>>();
            if active.is_empty() {
                let (level_dots, win_threshold) = level(self.level)?;
                let captured = level_dots - self
                    .dots
                    .iter()
                    .filter(|(_, d)| d.state == DotState::Floating)
                    .collect::<Vec<(&u8, &Dot)>>()
                    .len() as u8;
                if captured >= win_threshold {
                    self.level_state = LevelState::Won;
                } else {
                    self.level_state = LevelState::Lost;
                }
            }
        }
        Ok(())
    }

    // Array format:
    // [f32; 6]: level_number | level_state | total_dots | win_threshold | caputured_dots | last_update
    fn header(&self) -> Result<LevelHeader, String> {
        let (level_dots, win_threshold) = level(self.level)?;
        // grab the dots first, and then call separate filter and len on the local one instead of your total_dots if_else
        let captured = level_dots - self
            .dots
            .values()
            .filter(|d| d.state == DotState::Floating)
            .collect::<Vec<&Dot>>()
            .len() as u8;
        let total_dots = self.dots.len() as u8;
        let data_vec = vec![
            f32::from(self.level),
            f32::from(self.level_state as u8),
            f32::from(total_dots),
            f32::from(win_threshold),
            f32::from(captured),
            f32::from(self.last_update),
        ];
        let mut ret: [f32; 6] = [0.0; 6];
        ret[..6].copy_from_slice(&data_vec[..6]);
        Ok(ret)
    }
}

// (total_dots, win_threshold)
fn level(number: u8) -> Result<(u8, u8), String> {
    match number {
        1 => Ok((5, 1)),
        2 => Ok((10, 2)),
        3 => Ok((15, 3)),
        4 => Ok((20, 5)),
        5 => Ok((25, 7)),
        6 => Ok((30, 10)),
        7 => Ok((35, 15)),
        8 => Ok((40, 21)),
        9 => Ok((45, 27)),
        10 => Ok((50, 33)),
        11 => Ok((55, 44)),
        12 => Ok((60, 50)),
        _ => Err(format!("No level defined: {}", number)),
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LevelState {
    Begin = 0,
    Waiting = 1,
    Clicked = 2,
    Won = 3,
    Lost = 4,
}
