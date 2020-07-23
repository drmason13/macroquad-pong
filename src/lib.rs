pub mod consts;
pub mod pong;
pub mod systems;

pub use consts::*;
pub use pong::{Ball, Paddle, Side};
pub use systems::collision;

use macroquad::*;

pub struct State {
    pub game_left: f32,
    pub game_right: f32,
    pub game_top: f32,
    pub game_bottom: f32,
    pub score: (u8, u8),
    pub score_pos: (f32, f32),
    pub score_text: String,
}

impl State {
    pub fn new() -> Self {
        // cache (?) score text in state since we draw it every frame but change it much less frequently
        let score_text = format!("{:<3}:{:>3}", 0, 0);
        let game_left = screen_width() * 0.5 - GAME_WIDTH * 0.5;
        let game_right = screen_width() * 0.5 + GAME_WIDTH * 0.5;
        let game_top = screen_height() * 0.5 - GAME_HEIGHT * 0.5;
        let game_bottom = screen_height() * 0.5 + GAME_HEIGHT * 0.5;

        let (score_width, score_height) = measure_text(&score_text, SCORE_FONT_SIZE);

        State {
            game_left,
            game_right,
            game_top,
            game_bottom,
            score: (0, 0),
            score_pos: (game_left + GAME_WIDTH * 0.5 - score_width * 0.5, game_top - score_height * 2.0),
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

/// create the initial ball(s), spawned in the center of the game
pub fn init_balls(state: &mut State) -> Vec<Ball> {
    vec![Ball::new(
        state.game_left + GAME_WIDTH * 0.5,
        state.game_top + GAME_HEIGHT * 0.5,
        vec2(7.0, 5.0),
    )]
}

/// create the initial paddles, one left with "left" keybindings and one "right" likewise
pub fn init_paddles(state: &mut State) -> Vec<Paddle> {
    vec![
        Paddle::new(
            state.game_left + PADDLE_WIDTH * 2.0,
            state.game_top + GAME_HEIGHT * 0.5,
            Side::Left,
        ),
        Paddle::new(
            state.game_left + GAME_WIDTH - PADDLE_WIDTH * 2.0,
            state.game_top + GAME_HEIGHT * 0.5,
            Side::Right,
        ),
    ]
}
