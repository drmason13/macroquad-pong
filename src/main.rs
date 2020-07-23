use macroquad::*;

use pong::{State, Render, Update, collision::{bounce_paddle, bounce_wall, point_in_rect}};
use pong::{init_balls, init_paddles};
use pong::consts::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    // initialises a default state with screen width, height
    let mut state = State::new();

    let mut balls = init_balls(&mut state);
    let mut paddles = init_paddles(&mut state);

    loop {
        clear_background(BLACK);
        draw_rectangle_lines(X_OFFSET, Y_OFFSET, state.screen_width - 2.0 * X_OFFSET, state.screen_height - 2.0 * Y_OFFSET, PADDLE_WIDTH, WHITE);

        for e in &mut balls {
            e.draw();
            e.update(&mut state);
        }
        
        for e in &mut paddles {
            e.draw();
            e.update(&mut state);
            e.input(&(e.side).clone());
        }

        // bounce balls
        for ball in &mut balls {
            for paddle in &mut paddles {
                // bounce against paddles left and right
                if point_in_rect(ball.pos, paddle.x - PADDLE_WIDTH, paddle.y - PADDLE_HEIGHT, paddle.x + PADDLE_WIDTH, paddle.y + PADDLE_HEIGHT) {
                    bounce_paddle(ball, &paddle.side, &mut state);
                }
            }
            // bounce off top/bottom of screen up and down
            bounce_wall(ball, &mut state);
        }

        // draw score
        draw_text(&format!("{:<10}:{:>10}", state.score.0, state.score.1), state.screen_width * 0.5, Y_OFFSET + 10.0, 30.0, WHITE);
        
        next_frame().await
    }
}