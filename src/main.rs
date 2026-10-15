use macroquad::prelude::*;

#[macroquad::main("GameTutorial")]
async fn main() {
    let speed = 500.0;
    rand::srand(miniquad::date::now() as u64);
    let mut squares = Vec::new();
    let mut circle = Shape {
        size: 10.0,
        speed,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };

    loop {
        clear_background(DARKPURPLE);
        let delta_time = get_frame_time();
        if is_key_down(KeyCode::Left) {
            circle.move_left(delta_time);
        }
        if is_key_down(KeyCode::Right) {
            circle.move_right(delta_time);
        }
        if is_key_down(KeyCode::Up) {
            circle.move_up(delta_time);
        }
        if is_key_down(KeyCode::Down) {
            circle.move_down(delta_time);
        }
        if is_key_down(KeyCode::Escape) {
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
        }
        circle.x = circle.x.clamp(0.0, screen_width());
        circle.y = circle.y.clamp(0.0, screen_height());

        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size,
            });
        }
        for square in &mut squares {
            square.move_down(delta_time)
        }
        squares.retain(|square| square.y < screen_height() + square.size);
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                BLACK,
            );
        }
        draw_circle(circle.x, circle.y, circle.size, ORANGE);

        next_frame().await
    }
}

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}
impl Shape {
    fn move_down(&mut self, delta_time: f32) {
        self.y += self.speed * delta_time;
    }
    fn move_up(&mut self, delta_time: f32) {
        self.y -= self.speed * delta_time;
    }
    fn move_right(&mut self, delta_time: f32) {
        self.x += self.speed * delta_time;
    }
    fn move_left(&mut self, delta_time: f32) {
        self.x -= self.speed * delta_time;
    }
}
