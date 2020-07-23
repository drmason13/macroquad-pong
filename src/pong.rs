use macroquad::{draw_circle, draw_rectangle, vec2, Vec2, WHITE};
use macroquad::{is_key_down, KeyCode};

use crate::{consts::*, Render, State, Update};

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
        let up_key = if *side == Side::Right {
            KeyCode::Up
        } else {
            KeyCode::W
        };

        let down_key = if *side == Side::Right {
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
        draw_rectangle(
            self.x - PADDLE_WIDTH_HALF,
            self.y - PADDLE_HEIGHT_HALF,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
            WHITE,
        )
    }
}

impl Update for Paddle {
    fn update(&mut self, state: &mut State) {
        let top = state.game_top + PADDLE_HEIGHT_HALF;
        let bottom = state.game_bottom - PADDLE_HEIGHT_HALF;
        // clamp to game area
        if self.y <= top {
            self.y = top
        }
        if self.y >= bottom {
            self.y = bottom
        }
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
        Ball {
            pos: vec2(x, y),
            vel,
            timer: None,
            spawn: vec2(x, y),
        }
    }

    fn respawn(&mut self) {
        // respawn where we were first made
        self.pos = self.spawn;
        // freeze for a bit
        self.timer = Some(5.0);
        // reverse direction
        *self.vel.x_mut() *= -1.0;
    }
}

impl Render for Ball {
    fn draw(&self) {
        draw_circle(self.pos.x(), self.pos.y(), BALL_RADIUS, WHITE);
    }
}

impl Update for Ball {
    fn update(&mut self, state: &mut State) {
        // move
        if let Some(t) = self.timer {
            // TODO: actual you know, timing
            self.timer = if t <= 0.0 { None } else { Some(t - 0.3) };
        } else {
            // move if not frozen (currently the only use for timer)
            // TODO: add a Status enum so that we can do arbitrary things to balls for a duration using the same timer... hmm, thinking about that, we'll need multiple timers :(
            self.pos += self.vel;
        }
        // score and respawn
        let x = self.pos.x();

        let left = state.game_left + BALL_RADIUS;
        let right = state.game_right - BALL_RADIUS;

        if x < left {
            // off left side, right side scores
            state.score.1 += 1;
            state.score_text = format!("{:<3}:{:>3}", state.score.0, state.score.1);
            self.respawn();
        } else if x > right {
            // off right side, left side scores
            state.score.0 += 1;
            state.score_text = format!("{:<3}:{:>3}", state.score.0, state.score.1);
            self.respawn();
        }
    }
}
