use macroquad::prelude::*;

#[macroquad::main("GameTutorial")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    let speed = 1000.0;

    loop {
        clear_background(DARKPURPLE);
        let delta_time = get_frame_time();
        if is_key_down(KeyCode::Left) {
            x -= speed * delta_time
        }
        if is_key_down(KeyCode::Right) {
            x += speed * delta_time
        }
        if is_key_down(KeyCode::Up) {
            y -= speed * delta_time
        }
        if is_key_down(KeyCode::Down) {
            y += speed * delta_time
        }
        if is_key_down(KeyCode::Escape) {
            x = screen_width() / 2.0;
            y = screen_height() / 2.0;
        }
        x = clamp(x, 0.0, screen_width());
        y = clamp(y, 0.0, screen_height());
        draw_circle(x, y, 10.0, ORANGE);
        next_frame().await
    }
}
