use macroquad::prelude::*;

// So, since I'm using macroquad's prelude it imports a lot of different things for me.
// Macroquad uses glam for the Vec2, it also uses quad_rand for me 

pub const DELTA_T: f32 = 0.5;

pub const B_FIELD: f32 = 3.8;

pub const EM_CONST: f32 = 0.001;

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub size: f32,
    pub charge: i8,
}

pub fn spawn(v: Vec<Particle>) -> Vec<Particle> {
    
    let mut v_temp = v;

    v_temp.push(Particle {
        pos: Vec2::new(10.0, screen_height() / 2.0),
        vel: Vec2::new(rand::gen_range(0.6, 1.0), rand::gen_range(-0.5, 0.5)),
        size: 10.0,
        charge: rand::gen_range(-2, 2),
    });

    v_temp
}

pub fn despawn(v: &Particle) -> bool {

    let mut result: bool = true;

    if v.pos.x > screen_width() { result = false; }
    if v.pos.x < 0.0 { result = false; }
    if v.pos.y > screen_height() { result = false; }
    if v.pos.y < 0.0 { result = false; }
    result
}

pub fn lorentz_force(p: &mut Particle) {
    
    // Lorentz force: F = q * v × B
    // For 2D with B field in z-direction: F_x = c * q * v_y * B, F_y = c* -q * v_x * B
    let acc_x = EM_CONST * p.charge as f32 * p.vel.y * B_FIELD;
    let acc_y = EM_CONST * -(p.charge as f32) * p.vel.x * B_FIELD;
    
    // Update velocity
    p.vel.x += acc_x * DELTA_T;
    p.vel.y += acc_y * DELTA_T;
    
}

/* pub fn mom_exchange(p: &mut Particle) {
    
    if p.pos.x > (screen_width() / 2.0 - p.size) {
        p.vel.x = -p.vel.x;
    }
    if p.pos.y == 2.0 {
        p.vel.y = -p.vel.y;
    }
} */