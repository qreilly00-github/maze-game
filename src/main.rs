use notan::draw::*;
use notan::prelude::*;

pub mod viewport;
pub mod objects;

use crate::objects::Drawable;

#[notan_main]
fn main() -> Result<(), String> {
    let win_config = WindowConfig::new()
        .size(1280, 720)
        .vsync(true);

    notan::init_with(State::new)
        .add_config(win_config)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn update(app: &mut App, state: &mut State) {
    *state.main_view.window_size() = (app.window().width(), app.window().height());
}

fn draw(gfx: &mut Graphics, state: &mut State /*window_size: (i32, i32)*/) {
    let mut draw = gfx.create_draw();

    state.main_view.set(&mut draw);

    state.tmp.draw(&mut draw, (0.0, 0.0));

    draw.clear(Color::BLACK);
    gfx.render(&draw);
}

#[derive(AppState)]
struct State {
    main_view: viewport::Viewport,
    tmp: objects::Wall,
}

impl State {

    fn new() -> Self {
        Self {
            main_view: viewport::Viewport::new(),
            tmp: objects::Wall::new(),
        }
    }
}