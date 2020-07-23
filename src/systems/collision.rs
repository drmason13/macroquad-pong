use macroquad::{Vec2};

use super::super::{State, Ball, Side, consts::*};

// A point is in a box when its coordinates are smaller or equal than the top
// right and larger or equal than the bottom left.
pub fn point_in_rect(point: Vec2, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    let (x, y) = (point.x(), point.y());
    x >= left && x <= right && y >= bottom && y <= top
}

pub fn bounce_paddle(ball: &mut Ball, side: &Side, _state: &mut State) {
    if (*side == Side::Left && ball.vel.x() < 0.0)
        || (*side == Side::Right && ball.vel.x() > 0.0)
    {
        *ball.vel.x_mut() *= -1.0;
    }
}

pub fn bounce_wall(ball: &mut Ball, state: &mut State) {
    if (ball.pos.y() < Y_OFFSET + BALL_RADIUS && ball.vel.y() < 0.0)
        || (ball.pos.y() > state.screen_height - Y_OFFSET - BALL_RADIUS && ball.vel.y() > 0.0)
    {
        *ball.vel.y_mut() *= -1.0;
    }
}
