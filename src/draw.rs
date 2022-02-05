use piston_window::{types::Color, Context, G2d, Glyphs, Text, Transformed, Ellipse };

pub fn map(x: f64, max: f64) -> f64 {
    (x as f64) / 100.0 * max 
}

#[allow(dead_code)]
pub const LEFT: f64 = 0.0;
pub const CENTER: f64 = -0.5;
#[allow(dead_code)]
pub const RIGHT: f64 = -1.0;

pub const GOLD: Color = [1.0, 0.8, 0.0, 1.0];
pub const FONT_SIZE: f64 = 32.0;
pub const CHAR_WIDTH: f64 = FONT_SIZE / 2.0;

pub fn text(text: &str, color: Color, scale: u32, x: f64, y: f64, float: f64, c: Context, g: &mut G2d, glyphs: &mut Glyphs) {
    Text::new_color(color, FONT_SIZE as u32 * scale).draw(
        text, 
        glyphs, 
        &c.draw_state, 
        c.transform.trans(x + CHAR_WIDTH * text.len() as f64 * float, y + FONT_SIZE * -0.5), 
        g
    ).expect("Couldn't draw text");
}

pub fn ball(color: Color, radius: f64, x: f64, y: f64, c: Context, g: &mut G2d) {
    Ellipse::new(color).draw(
        [ 0.0, 0.0, radius, radius ], 
        &c.draw_state, 
        c.transform.trans(x, y), 
        g
    );
}