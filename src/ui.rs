use macroquad::prelude::*;

pub struct Button {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    text: String,
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, text: &str) -> Self {
        Button {
            x,
            y,
            width,
            height,
            text: text.to_string(),
        }
    }
    
    fn is_mouse_over(&self) -> bool {
        let (mouse_x, mouse_y) = mouse_position();
        mouse_x >= self.x && mouse_x <= self.x + self.width
            && mouse_y >= self.y && mouse_y <= self.y + self.height
    }
    
    pub fn draw(&self, is_active: bool) {
        // Draw button background
        let button_color = if self.is_mouse_over() { DARKGRAY } else { GRAY };
        draw_rectangle(self.x, self.y, self.width, self.height, button_color);
        
        // Draw text
        let text_color = if is_active { GREEN } else { RED };
        draw_text(&self.text, self.x + 10.0, self.y + 27.0, 20.0, text_color);
    }
    
    pub fn is_clicked(&self) -> bool {
        self.is_mouse_over() && is_mouse_button_pressed(MouseButton::Left)
    }
}