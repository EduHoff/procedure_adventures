use procedure_adventures::{
    core::{entities::Player, map::{Map, Tile}},
    display::terminal,
    engine::movement,
};

fn main() {
    let mut map = Map::new(20, 20);
    let mut player = Player { x: 5, y: 5 };

    for x in 0..10 {
        map.set_tile(x, 10, Tile::Wall);
    }

    let _guard = terminal::RawModeGuard::new();

    loop {
        terminal::render(&map, &player);
        
        if let Some(direction) = movement::get_input() {
            if direction == crossterm::event::KeyCode::Esc {
                break;
            }
            movement::handle_move(&mut player, &map, direction);
        }
    }

}
