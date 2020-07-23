use macroquad::*;

use pong::consts::*;
use pong::{
    collision::{bounce_paddle, bounce_wall},
    Render, State, Update,
};
use pong::{init_balls, init_paddles};

#[macroquad::main("Pong!")]
async fn main() {
    // initialises a state with screen width, height
    let mut state = State::new();

    let mut balls = init_balls(&mut state);
    let mut paddles = init_paddles(&mut state);

    loop {
        clear_background(BLACK);
        // draw the borders that balls and paddles bounce off around the game area
        draw_rectangle_lines(
            state.game_left,
            state.game_top,
            GAME_WIDTH,
            GAME_HEIGHT,
            PADDLE_WIDTH,
            WHITE,
        );

        for e in &mut balls {
            e.draw();
            e.update(&mut state);
        }

        for e in &mut paddles {
            e.draw();
            e.input(&(e.side).clone());
            e.update(&mut state);
        }

        // bounce balls
        for ball in &mut balls {
            for paddle in &mut paddles {
                // bounce against paddles left and right
                bounce_paddle(ball, paddle, &mut state);
            }
            // bounce off top/bottom of screen up and down
            bounce_wall(ball, &mut state);
        }

        // draw score
        draw_text(
            &state.score_text,
            state.score_pos.0,
            state.score_pos.1,
            SCORE_FONT_SIZE,
            WHITE,
        );

        next_frame().await
    }
}
