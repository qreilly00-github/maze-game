use notan::draw::*;
use notan::prelude::*;

pub mod viewport;
pub mod objects;

use crate::objects::Drawable;

#[derive(PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

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

    if app.keyboard.is_down(KeyCode::W) {
        state.tmp_player.move_pos(app.timer.delta_f32(), Direction::UP)
    }
    if app.keyboard.is_down(KeyCode::A) {
        state.tmp_player.move_pos(app.timer.delta_f32(), Direction::LEFT)
    }
    if app.keyboard.is_down(KeyCode::S) {
        state.tmp_player.move_pos(app.timer.delta_f32(), Direction::DOWN)
    }
    if app.keyboard.is_down(KeyCode::D) {
        state.tmp_player.move_pos(app.timer.delta_f32(), Direction::RIGHT)
    }
}

fn draw(gfx: &mut Graphics, state: &mut State /*window_size: (i32, i32)*/) {
    let mut draw = gfx.create_draw();

    state.main_view.set(&mut draw);

    state.tmp_wall.draw(&mut draw, (0.0, 0.0));
    state.tmp_player.draw(&mut draw, (0.0, 0.0));

    draw.clear(Color::BLACK);
    gfx.render(&draw);
}

#[derive(AppState)]
struct State {
    main_view: viewport::Viewport,

    tmp_wall: objects::Wall,
    tmp_player: objects::Player,
}

impl State {

    fn new() -> Self {
        Self {
            main_view: viewport::Viewport::new(),

            tmp_wall: objects::Wall::new(),
            tmp_player: objects::Player::new(),
        }
    }
}