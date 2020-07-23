use macroquad::Vec2;

use super::super::{consts::*, Ball, Paddle, Side, State};

// A point is in a box when its coordinates are smaller or equal than the top
// right and larger or equal than the bottom left.
pub fn point_in_rect(point: Vec2, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    let (x, y) = (point.x(), point.y());
    x >= left && x <= right && y >= bottom && y <= top
}

pub fn bounce_paddle(ball: &mut Ball, paddle: &mut Paddle, _state: &mut State) {
    if (paddle.side == Side::Left && ball.vel.x() < 0.0)
        || (paddle.side == Side::Right && ball.vel.x() > 0.0)
    {
        if point_in_rect(
            ball.pos,
            paddle.x - PADDLE_WIDTH_HALF - BALL_RADIUS,
            paddle.y - PADDLE_HEIGHT_HALF - BALL_RADIUS,
            paddle.x + PADDLE_WIDTH_HALF + BALL_RADIUS,
            paddle.y + PADDLE_HEIGHT_HALF + BALL_RADIUS,
        ) {
            *ball.vel.x_mut() *= -1.0;
        }
    }
}

pub fn bounce_wall(ball: &mut Ball, state: &mut State) {
    let top = state.game_top + BALL_RADIUS;
    let bottom = state.game_bottom - BALL_RADIUS;
    if (ball.pos.y() < top && ball.vel.y() < 0.0) || (ball.pos.y() > bottom && ball.vel.y() > 0.0) {
        *ball.vel.y_mut() *= -1.0;
    }
}
