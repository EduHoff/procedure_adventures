use procedure_adventures::{
    core::{entities::Player, map::Map},
    display::terminal,
};

fn main() {
    let map = Map::new(20, 20);
    let player = Player { x: 5, y: 5 };

    terminal::render(&map, &player);
}
