use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(GRAY);
        
        draw_circle(40.0, 40.0, 20.0, YELLOW);

        next_frame().await
    }
}