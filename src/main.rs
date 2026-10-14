use macroquad::prelude::*;

#[macroquad::main("GameTutorial")]
async fn main() {
    loop {
        clear_background(DARKPURPLE);
        next_frame().await
    }
}
