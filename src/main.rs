extern crate piston_window;
extern crate find_folder;
extern crate rand;

mod game;
mod draw;

use game::App;
use piston_window::*;

const WIDTH: f64 = 800.;
const HEIGHT: f64 = 450.;

fn main() {

    let mut window: PistonWindow = WindowSettings::new("Fong", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .expect("Couldn't create window");

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let ref font = assets.join("vt323.ttf");
    let factory = window.create_texture_context();
    let settings = TextureSettings::new();
    let mut glyphs = Glyphs::new(font, factory, settings).unwrap();

    let mut app = App::new(WIDTH, HEIGHT);

    while let Some(event) = window.next() {
        event.update(|args| {
            app.update(args.dt as u32);
        });

        event.render(|_| window.draw_2d(&event, |c, g, device| {
            app.render(c, g, &mut glyphs);
            glyphs.factory.encoder.flush(device);
        }));

        event.button(|args| {
            match args.button {
                Button::Keyboard(key) => app.keyboard(key),
                _ => (),
            }
        });

        event.resize(|args| {
            app.resize(
                args.draw_size[0] as f64, 
                args.draw_size[1] as f64,
            );
        });
    }
}
