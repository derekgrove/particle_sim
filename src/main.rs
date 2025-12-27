use macroquad::prelude::*;

const DELTA_T: f32 = 0.5;


struct Particle {
    pos: Vec2,
    vel: Vec2,
    size: f32,
    charge: i8,
}

fn wrap_around(v: &Vec2) -> Vec2 {
    let mut vr = *v;
    if vr.x > screen_width() { vr.x = 0.0; }
    if vr.x < 0.0 { vr.x = screen_width(); }
    if vr.y > screen_height() { vr.y = 0.0; }
    if vr.y < 0.0 { vr.y = screen_height(); }
    vr
}

// fn mom_exchange(p: &mut Particle) {
//    
//    if p.pos.x > (screen_width() / 2.0 - p.size) {
//        p.vel.x = -p.vel.x;
//    }
//    if p.pos.y == 2.0 {
//        p.vel.y = -p.vel.y;
//    }
//}


#[macroquad::main("Particles")]
async fn main() {
    let mut particles: Vec<Particle> = Vec::new();
    let screen_edge = Vec2::new(10.0, screen_height() / 2.0);

    // spawn once
    for _ in 0..10 {
        particles.push(Particle {
            pos: screen_edge,
            vel: Vec2::new(rand::gen_range(0.1, 1.0), rand::gen_range(-0.5, 0.5)),
            size: 20.0,
            charge: 1,
        });
    }

    loop {
        clear_background(LIGHTGRAY);

        // update + draw
        for p in particles.iter_mut() {
            //mom_exchange(p);
            p.pos += p.vel * DELTA_T;
            p.pos = wrap_around(&p.pos);
            

            draw_circle(p.pos.x, p.pos.y, p.size, BLACK);
        }

        //draw_rectangle(screen_width() / 2.0, 0.0, 2.0, 2000.0, BLACK);

        next_frame().await;
    }
}
