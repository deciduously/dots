// game.rs contains the game logic
use super::{FINAL_RADIUS, SCREEN_SIZE, SPEED, START_RADIUS};
use ffi::{now, random};
use std::{collections::HashMap, fmt};

// UTILITY

// Create initial dot layout
fn init_dots(l: u16) -> Result<HashMap<u16, Dot>, ::std::io::Error> {
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
                // hang out at full for 200ms
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
// level_number | total_dots | win_threshold | caputured_dots | last_update
pub type LevelHeader = [f32; 5];

pub struct Level {
    dots: HashMap<u16, Dot>,
    last_update: u16,
    pub level: u16,
    clicked: bool,
}

impl Level {
    // Public

    pub fn new(l: u16) -> Result<Level, ::std::io::Error> {
        Ok(Level {
            dots: init_dots(l)?,
            last_update: now(),
            level: l,
            clicked: false,
        })
    }

    pub fn tick(&mut self) {
        for d in self.dots.values_mut() {
            d.tick();
        }
        self.handle_collisions();
        self.last_update = now();
    }

    pub fn add_player(&mut self, x: f32, y: f32) {
        if !self.clicked {
            let idx = self.dots.len() as u16;
            self.dots.insert(
                idx,
                Dot::new((x, y).into(), Point::new(0.0, 0.0), DotState::Growing),
            );
            self.clicked = true;
        }
    }

    pub fn restart_level(&mut self) -> Result<(), ::std::io::Error> {
        self.dots = init_dots(self.level)?;
        self.clicked = false;
        self.last_update = now();
        Ok(())
    }

    pub fn pack(&self) -> Result<Vec<f32>, ::std::io::Error> {
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

    fn capture_dot(&mut self, id: u16) {
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

    // Array format:
    // [f32; 5]: level_number | total_dots | win_threshold | caputured_dots | last_update
    fn header(&self) -> Result<LevelHeader, ::std::io::Error> {
        let (level_dots, win_threshold) = level(self.level)?;
        let captured = level_dots - self
            .dots
            .iter()
            .filter(|(_, d)| d.state == DotState::Floating)
            .collect::<Vec<(&u16, &Dot)>>()
            .len() as u16;
        let total_dots = if self.clicked {
            level_dots + 1
        } else {
            level_dots
        };
        let mut ret: [f32; 5] = [0.0; 5];
        ret[0] = f32::from(self.level);
        ret[1] = f32::from(total_dots);
        ret[2] = f32::from(win_threshold);
        ret[3] = f32::from(captured);
        ret[4] = f32::from(self.last_update);
        Ok(ret)
    }
}

// (total_dots, win_threshold)
fn level(number: u16) -> Result<(u16, u16), ::std::io::Error> {
    match number {
        1 => Ok((5, 1)),
        2 => Ok((10, 3)),
        3 => Ok((15, 4)),
        4 => Ok((20, 7)),
        5 => Ok((30, 10)),
        6 => Ok((40, 15)),
        7 => Ok((60, 40)),
        _ => Err(::std::io::Error::new(
            ::std::io::ErrorKind::InvalidInput,
            format!("No level defined: {}", number),
        )),
    }
}

//#[repr(u8)]
//#[derive(Clone, Copy, Debug, PartialEq, Eq)]
//enum LevelState {
//    Waiting = 0,
//    Clicked = 1,
//}
