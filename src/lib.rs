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
    pub score: (u8, u8),
    pub score_pos: (f32, f32),
    pub score_text: String,
}

impl State {
    pub fn new() -> Self {
        // cache (?) score text in state since we draw it every frame but change it much less frequently
        let score_text = format!("{:<3}:{:>3}", 0, 0);  
        State {
            screen_height: screen_height(),
            screen_width: screen_width(),
            score: (0, 0),
            score_pos: (
                screen_width() * 0.5 - measure_text(&score_text, SCORE_FONT_SIZE).0 * 0.5,  // x
                Y_OFFSET + PADDLE_WIDTH                                                     // y
            ),
            score_text,
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
