use std::f64::consts::PI;

use piston_window::{Context, G2d, clear, Glyphs, Key };
use rand::{thread_rng, Rng};

use crate::{draw::{ GOLD, CENTER, self, map }};

pub enum GameState {
    Init,
    Running {
        elapsed_time: u32,
        balls: Vec<Ball>,
        bat: u32,
        score: u8,
    },
    Result {
        elapsed_time: u32,
        score: u8,
    }
}

impl GameState {
    pub fn start() -> Self {
        let mut rng = thread_rng();

        Self::Running {
            elapsed_time: 0,
            balls: vec![
                Ball {
                    pos: [50., 80.],
                    speed: 0.6,
                    orientation: rng.gen_range(-45..=45) as f64 / 180. * PI,
                    radius: 3.,
                }
            ],
            bat: 225,
            score: 0,
        }
    }
}

pub struct Ball {
    pos: [f64; 2],
    speed: f64,
    orientation: f64,
    radius: f64,
}

impl Ball {
    pub fn update(&self) -> Self {
        let mut rng = thread_rng();

        let orientation = if self.pos[0] <= 0. || self.pos[1] <= 0. || self.pos[0] >= 100. - self.radius {
            self.orientation + rng.gen_range(-60..60) as f64
        } else {
            self.orientation
        };

        let speed_x = self.speed * orientation.sin(); 
        let speed_y = -self.speed * orientation.cos(); 

        Self {
            pos: [ self.pos[0] + speed_x, self.pos[1] + speed_y ],
            speed: self.speed,
            orientation: orientation,
            radius: self.radius,
        }
    }
}

pub struct App {
    game_state: GameState,
    width: f64,
    height: f64,
}

impl App {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            game_state: GameState::Init,
            width,
            height,
        }
    }

    pub fn keyboard(&mut self, key: Key) {
        match (key, &self.game_state) {
            (Key::Space, GameState::Init) => {
                self.game_state = GameState::start();
            }
            _ => (),
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d, glyphs: &mut Glyphs) {;
        clear([0.2, 0.2, 0.2, 1.0], g);

        match &self.game_state {
            GameState::Init => {
                draw::text("Press <Space> to start", GOLD, 1,
                    map(50., self.width), map(50., self.height), CENTER,
                    c, g, glyphs
                );
            },
            GameState::Running { balls, bat, score, .. } => {
                for ball in balls {
                    draw::ball(
                        GOLD,
                        map(ball.radius, self.width), 
                        map(ball.pos[0], self.width), 
                        map(ball.pos[1], self.height),
                        c, g
                    );
                }
            },
            GameState::Result { elapsed_time, score } => todo!(),
        }
    }

    pub fn resize(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }

    pub fn update(&mut self, dt: u32) {
        match &self.game_state {
            GameState::Running { elapsed_time, balls, bat, score } => {
                let mut new_balls: Vec<Ball> = Vec::new();
                for ball in balls {
                    let new_ball = ball.update();
                    if new_ball.pos[1] >= 100.0 {
                        self.game_state = GameState::Result {
                            elapsed_time: elapsed_time + dt,
                            score: *score,
                        };
                        return;
                    }
                    new_balls.push(new_ball);
                }

                self.game_state = GameState::Running {
                    elapsed_time: elapsed_time + dt,
                    balls: new_balls,
                    bat: *bat,
                    score: *score,
                }
            },
            _ => (),
        }
    }
}