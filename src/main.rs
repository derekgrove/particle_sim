mod particle;
mod ui;

use macroquad::prelude::*;
use particle::*;
use ui::Button;

const NUM_PART: i32 = 290;

#[macroquad::main("Particles")]
async fn main() {

    let mut particles: Vec<Particle> = Vec::new();
    let mut b_field_on = true;

    loop {
        clear_background(LIGHTGRAY);

        if particles.len() < NUM_PART as usize { particles = spawn(particles); }
        
        //update position, draw particles
        for p in particles.iter_mut() {

            p.pos += p.vel * DELTA_T;
            if b_field_on { lorentz_force(p); }
            

            let color = if p.charge < 0 {
                    RED
                } else if p.charge > 0 {
                    BLUE
                } else {
                    BLACK
                };

            draw_circle(p.pos.x, p.pos.y, p.size, color);
        }

        //check the particle array, if any pass the despawn function (go off screen) remove them from array
        particles.retain(|particle| despawn(&particle));

        // Draw and handle button
        let button_text = if b_field_on { "B-Field: ON" } else { "B-Field: OFF" };
        let button = Button::new(screen_width() - 150.0, 20.0, 130.0, 40.0, button_text);
        button.draw(b_field_on);
        
        if button.is_clicked() {
            b_field_on = !b_field_on;
        }

        
        

        let part_count = format!("num of particles: {}", particles.len());

        draw_text(&part_count, 20.0, 30.0, 32.0, BLACK);

        //draw_rectangle(screen_width() / 2.0, 0.0, 2.0, 2000.0, BLACK);
        next_frame().await;
    }
}