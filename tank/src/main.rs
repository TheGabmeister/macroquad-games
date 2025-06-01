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
    loop {
        clear_background(BLACK);

        next_frame().await
    }
}

