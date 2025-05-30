use macroquad::prelude::*;

const PADDLE_WIDTH: f32 = 120.0;
const PADDLE_HEIGHT: f32 = 20.0;
const PADDLE_SPEED: f32 = 500.0;

const BALL_SIZE: f32 = 16.0;
const BALL_SPEED: f32 = 400.0;

const BRICK_ROWS: usize = 6;
const BRICK_COLS: usize = 10;
const BRICK_WIDTH: f32 = 60.0;
const BRICK_HEIGHT: f32 = 30.0;
const BRICK_PADDING: f32 = 8.0;
const BRICK_OFFSET_TOP: f32 = 60.0;
const BRICK_OFFSET_LEFT: f32 = 32.0;

struct Ball {
    pos: Vec2,
    vel: Vec2,
}

struct Paddle {
    x: f32,
}

struct Brick {
    rect: Rect,
    alive: bool,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Breakout".to_owned(),
        window_width: 720,
        window_height: 900,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut paddle = Paddle {
        x: screen_width() / 2.0 - PADDLE_WIDTH / 2.0,
    };

    let mut ball = Ball {
        pos: vec2(screen_width() / 2.0, screen_height() - 100.0),
        vel: vec2(BALL_SPEED, -BALL_SPEED),
    };

    let mut bricks = create_bricks();

    let mut game_over = false;
    let mut win = false;

    loop {
        clear_background(BLACK);

        // Paddle movement
        if is_key_down(KeyCode::Left) {
            paddle.x -= PADDLE_SPEED * get_frame_time();
        }
        if is_key_down(KeyCode::Right) {
            paddle.x += PADDLE_SPEED * get_frame_time();
        }
        paddle.x = paddle.x.clamp(0.0, screen_width() - PADDLE_WIDTH);

        // Ball movement
        if !game_over && !win {
            ball.pos += ball.vel * get_frame_time();

            // Wall collision
            if ball.pos.x <= 0.0 || ball.pos.x + BALL_SIZE >= screen_width() {
                ball.vel.x *= -1.0;
            }
            if ball.pos.y <= 0.0 {
                ball.vel.y *= -1.0;
            }

            // Paddle collision
            let paddle_rect = Rect::new(paddle.x, screen_height() - 40.0, PADDLE_WIDTH, PADDLE_HEIGHT);
            let ball_rect = Rect::new(ball.pos.x, ball.pos.y, BALL_SIZE, BALL_SIZE);
            if paddle_rect.overlaps(&ball_rect) && ball.vel.y > 0.0 {
                ball.vel.y *= -1.0;
                // Add some control based on where the ball hits the paddle
                let hit_pos = (ball.pos.x + BALL_SIZE / 2.0) - (paddle.x + PADDLE_WIDTH / 2.0);
                ball.vel.x = hit_pos * 6.0;
            }

            // Brick collision
            for brick in bricks.iter_mut() {
                if brick.alive && brick.rect.overlaps(&ball_rect) {
                    brick.alive = false;
                    ball.vel.y *= -1.0;
                    break;
                }
            }

            // Lose condition
            if ball.pos.y > screen_height() {
                game_over = true;
            }

            // Win condition
            if bricks.iter().all(|b| !b.alive) {
                win = true;
            }
        }

        // Draw bricks
        for brick in &bricks {
            if brick.alive {
                draw_rectangle(brick.rect.x, brick.rect.y, brick.rect.w, brick.rect.h, ORANGE);
            }
        }

        // Draw paddle
        draw_rectangle(paddle.x, screen_height() - 40.0, PADDLE_WIDTH, PADDLE_HEIGHT, BLUE);

        // Draw ball
        draw_circle(ball.pos.x + BALL_SIZE / 2.0, ball.pos.y + BALL_SIZE / 2.0, BALL_SIZE / 2.0, YELLOW);

        // Draw text
        if game_over {
            draw_text("GAME OVER", screen_width() / 2.0 - 120.0, screen_height() / 2.0, 60.0, RED);
            draw_text("Press R to Restart", screen_width() / 2.0 - 140.0, screen_height() / 2.0 + 60.0, 40.0, WHITE);
            if is_key_pressed(KeyCode::R) {
                paddle.x = screen_width() / 2.0 - PADDLE_WIDTH / 2.0;
                ball.pos = vec2(screen_width() / 2.0, screen_height() - 100.0);
                ball.vel = vec2(BALL_SPEED, -BALL_SPEED);
                bricks = create_bricks();
                game_over = false;
                win = false;
            }
        } else if win {
            draw_text("YOU WIN!", screen_width() / 2.0 - 100.0, screen_height() / 2.0, 60.0, GREEN);
            draw_text("Press R to Restart", screen_width() / 2.0 - 140.0, screen_height() / 2.0 + 60.0, 40.0, WHITE);
            if is_key_pressed(KeyCode::R) {
                paddle.x = screen_width() / 2.0 - PADDLE_WIDTH / 2.0;
                ball.pos = vec2(screen_width() / 2.0, screen_height() - 100.0);
                ball.vel = vec2(BALL_SPEED, -BALL_SPEED);
                bricks = create_bricks();
                game_over = false;
                win = false;
            }
        }

        next_frame().await
    }
}

fn create_bricks() -> Vec<Brick> {
    let mut bricks = Vec::new();
    for row in 0..BRICK_ROWS {
        for col in 0..BRICK_COLS {
            let x = BRICK_OFFSET_LEFT + col as f32 * (BRICK_WIDTH + BRICK_PADDING);
            let y = BRICK_OFFSET_TOP + row as f32 * (BRICK_HEIGHT + BRICK_PADDING);
            bricks.push(Brick {
                rect: Rect::new(x, y, BRICK_WIDTH, BRICK_HEIGHT),
                alive: true,
            });
        }
    }
    bricks
}
