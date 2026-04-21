use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use procedure_adventures::{
    core::{entities::Player, map::Map},
    display::terminal,
    engine::movement,
};

fn main() {
    let map = Map::new(20, 20);
    let mut player = Player { x: 5, y: 5 };

    enable_raw_mode().expect("Error activating Raw Mode.");

    loop {
        terminal::render(&map, &player);

        if let Some(direction) = movement::get_input() {
            if direction == crossterm::event::KeyCode::Esc {
                break;
            }
            movement::handle_move(&mut player, direction);
        }
    }

    disable_raw_mode().expect("Error disabling Raw Mode");
}
