use macroquad::prelude::*;

// So, since I'm using macroquad's prelude it imports a lot of different things for me.
// Macroquad uses glam for the Vec2, it also uses quad_rand for me 

const DELTA_T: f32 = 0.5;

const B_FIELD: f32 = 3.8;

static mut B_FIELD_ON: bool = true;

struct Particle {
    pos: Vec2,
    vel: Vec2,
    size: f32,
    charge: i8,
}

fn spawn(v: Vec<Particle>) -> Vec<Particle> {
    
    let mut v_temp = v;

    v_temp.push(Particle {
        pos: Vec2::new(10.0, screen_height() / 2.0),
        vel: Vec2::new(rand::gen_range(0.6, 1.0), rand::gen_range(-0.5, 0.5)),
        size: 10.0,
        charge: rand::gen_range(-2, 2),
    });

    v_temp
}

fn despawn(v: &Particle) -> bool {

    let mut result: bool = true;

    if v.pos.x > screen_width() { result = false; }
    if v.pos.x < 0.0 { result = false; }
    if v.pos.y > screen_height() { result = false; }
    if v.pos.y < 0.0 { result = false; }
    result
}

/* fn lorentz_force(p: Particle) -> Particle {
    let mut temp_p = p;

    mag_xy_vel = p.vel.x 
    acc = B_FIELD * temp_p.charge;
    
    temp_p.vel
    temp_p
} */

/* fn mom_exchange(p: &mut Particle) {
    
    if p.pos.x > (screen_width() / 2.0 - p.size) {
        p.vel.x = -p.vel.x;
    }
    if p.pos.y == 2.0 {
        p.vel.y = -p.vel.y;
    }
} */


#[macroquad::main("Particles")]
async fn main() {

    let mut particles: Vec<Particle> = Vec::new();
    
    loop {
        clear_background(LIGHTGRAY);
        if particles.len() < 290 { particles = spawn(particles); }
        
        //update position, draw particles
        for p in particles.iter_mut() {
            p.pos += p.vel * DELTA_T;
            let color = if p.charge < 0 {
                    RED
                } else if p.charge > 0 {
                    BLUE
                } else {
                    BLACK
                };
            draw_circle(p.pos.x, p.pos.y, p.size, color);
        }
        particles.retain(|particle| despawn(&particle));

        let part_count = format!("num of particles: {}", particles.len());

        draw_text(&part_count, 20.0, 30.0, 32.0, BLACK);

        //draw_rectangle(screen_width() / 2.0, 0.0, 2.0, 2000.0, BLACK);
        next_frame().await;
    }
}
