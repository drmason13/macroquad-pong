pub mod consts;
pub mod pong;
pub mod systems;

pub use consts::*;
pub use pong::{Ball, Paddle, Side};
pub use systems::collision;

use macroquad::*;

pub struct State {
    pub screen_height: f32,
    pub screen_width: f32,
    pub score: (u32, u32),
    pub score_text: String,
    pub font_size: f32,
}

impl State {
    pub fn new(score_text: String, font_size: f32) -> Self {
        State {
            screen_height: screen_height(),
            screen_width: screen_width(),
            score: (0, 0),
            score_text,
            font_size, 
        }
    }
}

pub trait Render {
    // each entity will call the right draw function for drawing itself
    fn draw(&self);
}

pub trait Update {
    // each entity will mutate itself to do it's normal thing
    fn update(&mut self, state: &mut State);
}

pub fn init_balls(state: &mut State) -> Vec<Ball> {
    vec![Ball::new(
        state.screen_width * 0.5,
        state.screen_height * 0.5,
        vec2(7.0, 5.0),
    )]
}

pub fn init_paddles(state: &mut State) -> Vec<Paddle> {
    vec![
        Paddle::new(
            X_OFFSET + PADDLE_WIDTH * 2.0,
            state.screen_height * 0.5,
            Side::Left,
        ),
        Paddle::new(
            state.screen_width - X_OFFSET - PADDLE_WIDTH * 2.0,
            state.screen_height * 0.5,
            Side::Right,
        ),
    ]
}
