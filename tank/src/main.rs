use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tank".to_owned(),
        window_width: 720,
        window_height: 900,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let controls1 = Controls {
        up: KeyCode::W,
        down: KeyCode::S,
        left: KeyCode::A,
        right: KeyCode::D,
        fire: KeyCode::Space,
    };
    let controls2 = Controls {
        up: KeyCode::Up,
        down: KeyCode::Down,
        left: KeyCode::Left,
        right: KeyCode::Right,
        fire: KeyCode::Enter,
    };

    let mut tanks = vec![
        Tank {
            pos: vec2(150.0, 450.0),
            rot: 0.0,
            color: GREEN,
            controls: controls1,
            cooldown: 0.0,
        },
        Tank {
            pos: vec2(570.0, 450.0),
            rot: 180.0,
            color: RED,
            controls: controls2,
            cooldown: 0.0,
        },
    ];

    let mut bullets: Vec<Bullet> = Vec::new();

    loop {
        clear_background(DARKGRAY);

        // Update tanks
        for (i, tank) in tanks.iter_mut().enumerate() {
            tank.update();
            // Fire
            if is_key_pressed(tank.controls.fire) && tank.cooldown <= 0.0 {
                let dir = vec2(tank.rot.to_radians().sin(), -tank.rot.to_radians().cos());
                let bullet_pos = tank.pos + dir * (TANK_SIZE.y / 2.0 + BULLET_SIZE.y / 2.0);
                bullets.push(Bullet {
                    pos: bullet_pos,
                    rot: tank.rot,
                    owner: i,
                });
                tank.cooldown = 0.5;
            }
        }

        // Update bullets
        for bullet in bullets.iter_mut() {
            bullet.update();
        }
        bullets.retain(|b| !b.is_offscreen());

        // Collision detection (simple)
        let mut reset_indices = Vec::new();
        for (i, tank) in tanks.iter().enumerate() {
            for bullet in bullets.iter() {
                if bullet.owner != i
                    && tank.pos.distance(bullet.pos) < (TANK_SIZE.x + BULLET_SIZE.x) / 2.0
                {
                    reset_indices.push(i);
                }
            }
        }
        for i in reset_indices {
            tanks[i].pos = if i == 0 {
                vec2(150.0, 450.0)
            } else {
                vec2(570.0, 450.0)
            };
        }

        // Draw tanks and bullets
        for tank in tanks.iter() {
            tank.draw();
        }
        for bullet in bullets.iter() {
            bullet.draw();
        }

        draw_text("Player 1: WASD + Space", 10.0, 30.0, 24.0, WHITE);
        draw_text("Player 2: Arrows + Enter", 400.0, 30.0, 24.0, WHITE);

        next_frame().await;
    }
}

const TANK_SIZE: Vec2 = Vec2::new(40.0, 60.0);
const BULLET_SIZE: Vec2 = Vec2::new(8.0, 16.0);
const BULLET_SPEED: f32 = 8.0;
const TANK_SPEED: f32 = 3.0;
const ROT_SPEED: f32 = 3.0;

struct Tank {
    pos: Vec2,
    rot: f32,
    color: Color,
    controls: Controls,
    cooldown: f32,
}

struct Bullet {
    pos: Vec2,
    rot: f32,
    owner: usize,
}

struct Controls {
    up: KeyCode,
    down: KeyCode,
    left: KeyCode,
    right: KeyCode,
    fire: KeyCode,
}

impl Tank {
    fn update(&mut self) {
        if is_key_down(self.controls.left) {
            self.rot -= ROT_SPEED;
        }
        if is_key_down(self.controls.right) {
            self.rot += ROT_SPEED;
        }
        let dir = vec2(self.rot.to_radians().sin(), -self.rot.to_radians().cos());
        if is_key_down(self.controls.up) {
            self.pos += dir * TANK_SPEED;
        }
        if is_key_down(self.controls.down) {
            self.pos -= dir * TANK_SPEED;
        }
        // Clamp to screen
        self.pos.x = self.pos.x.clamp(TANK_SIZE.x / 2.0, screen_width() - TANK_SIZE.x / 2.0);
        self.pos.y = self.pos.y.clamp(TANK_SIZE.y / 2.0, screen_height() - TANK_SIZE.y / 2.0);

        if self.cooldown > 0.0 {
            self.cooldown -= get_frame_time();
        }
    }

    fn draw(&self) {
        draw_rectangle_ex(
            self.pos.x - TANK_SIZE.x / 2.0,
            self.pos.y - TANK_SIZE.y / 2.0,
            TANK_SIZE.x,
            TANK_SIZE.y,
            DrawRectangleParams {
                offset: vec2(0.0, 0.0),
                rotation: self.rot.to_radians(),
                color: self.color,
            },
        );
        // Draw barrel
        let barrel_len = TANK_SIZE.y / 2.0 + 10.0;
        let barrel_end = self.pos
            + vec2(self.rot.to_radians().sin(), -self.rot.to_radians().cos()) * barrel_len;
        draw_line(self.pos.x, self.pos.y, barrel_end.x, barrel_end.y, 6.0, self.color);
    }
}

impl Bullet {
    fn update(&mut self) {
        let dir = vec2(self.rot.to_radians().sin(), -self.rot.to_radians().cos());
        self.pos += dir * BULLET_SPEED;
    }

    fn draw(&self) {
        draw_rectangle_ex(
            self.pos.x - BULLET_SIZE.x / 2.0,
            self.pos.y - BULLET_SIZE.y / 2.0,
            BULLET_SIZE.x,
            BULLET_SIZE.y,
            DrawRectangleParams {
                offset: vec2(0.0, 0.0),
                rotation: self.rot.to_radians(),
                color: YELLOW,
            },
        );
    }

    fn is_offscreen(&self) -> bool {
        self.pos.x < 0.0 || self.pos.x > screen_width() || self.pos.y < 0.0 || self.pos.y > screen_height()
    }
}
