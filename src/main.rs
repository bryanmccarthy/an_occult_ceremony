use macroquad::prelude::*;

#[macroquad::main("an occult ceremony")]
async fn main() {
    
    loop {
        clear_background(BLACK);
        draw_text("an occult ceremony", 20.0, 20.0, 30.0, WHITE);
        next_frame().await
    }
}
