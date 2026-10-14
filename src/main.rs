use macroquad::prelude::*;

#[macroquad::main("GameTutorial")]
async fn main() {
    loop {
        clear_background(DARKPURPLE);
        draw_text(
            "Hello World!",
            100.0,
            100.0,
            36.0,
            Color::from_hex(0xffffff),
        );
        next_frame().await
    }
}
