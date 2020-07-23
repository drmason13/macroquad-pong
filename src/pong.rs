use macroquad::{Vec2, vec2, draw_rectangle, draw_circle, draw_text, WHITE, GREEN};
use macroquad::{is_key_down, KeyCode};

use crate::{State, Render, Update, consts::*};

#[derive(PartialEq, Clone)]
pub enum Side {
    Left,
    Right,
    Up,
    Down,
}

pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub side: Side,
}

impl Paddle {
    pub fn new(x: f32, y: f32, side: Side) -> Self {
        Paddle { x, y, side }
    }

    pub fn input(&mut self, side: &Side) {
        let up_key = if *side == Side::Left {
            KeyCode::Up
        } else {
            KeyCode::W
        };
        
        let down_key = if *side == Side::Left {
            KeyCode::Down
        } else {
            KeyCode::S
        };

        // listen for user input
        if is_key_down(up_key) {
            self.y -= 6.0;
        } else if is_key_down(down_key) {
            self.y += 6.0;
        }
    }
}

impl Render for Paddle {
    fn draw(&self) {
        // draw centered on x, y
        draw_rectangle(self.x - PADDLE_WIDTH_HALF, self.y - PADDLE_HEIGHT_HALF, PADDLE_WIDTH, PADDLE_HEIGHT, WHITE)
    }
}

impl Update for Paddle {
    fn update(&mut self, state: &mut State) {
        let screen_height = state.screen_height;
        // clamp to screen
        if self.y <= Y_OFFSET { self.y = Y_OFFSET }
        if self.y >= screen_height - Y_OFFSET { self.y = screen_height - Y_OFFSET }
    }
}

#[derive(Debug)]
pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
    pub timer: Option<f32>,
    pub spawn: Vec2,
}

impl Ball {
    pub fn new(x: f32, y: f32, vel: Vec2) -> Self {
        Ball { pos: vec2(x, y), vel, timer: None, spawn: vec2(x, y) }
    }
}

impl Render for Ball {
    fn draw(&self) {
        draw_circle(self.pos.x(), self.pos.y(), BALL_RADIUS, WHITE);
        // DEBUG!
        draw_text(&format!("{:#?}", self), 0.0, Y_OFFSET + 10.0, 20.0, GREEN);
    }
}

impl Update for Ball {
    fn update(&mut self, state: &mut State) {
        // move
        if let Some(t) = self.timer {
            // TODO: actual you know, timing
            self.timer = if t <= 0.0 {
                None
            } else {
                Some(t - 0.3)
            };
        } else {
            // move if not frozen (currently the only use for timer)
            // TODO: add a Status enum so that we can do arbitrary things to balls for a duration using the same timer... hmm, thinking about that, we'll need multiple timers :(
            self.pos += self.vel;
        }
        // score and respawn
        let x = self.pos.x();

        // respawn
        let mut respawn = || {
            // respawn where we were first made
            self.pos = self.spawn;
            // freeze for a bit
            self.timer = Some(5.0);
            // reverse direction
            *self.vel.x_mut() *= -1.0;
        };

        if x < X_OFFSET {
            // off left side, right side scores
            state.score.1 += 1;
            respawn();
        } else if x > state.screen_width - X_OFFSET {
            // off right side, left side scores
            state.score.0 += 1;
            respawn();
        }
    }
}