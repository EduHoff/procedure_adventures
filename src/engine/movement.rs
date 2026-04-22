use crossterm::event::{Event, KeyCode, read};

use crate::core::{entities::Player, map::{Map}};

pub fn get_input() -> Option<KeyCode> {
    if let Ok(Event::Key(key_event)) = read() {
        return Some(key_event.code);
    }
    None
}

pub fn handle_move(player: &mut Player, map: &Map, key: KeyCode) {
    match key {
        KeyCode::Up | KeyCode::Char('w') => move_up(player),
        KeyCode::Down | KeyCode::Char('s') => move_down(player, map),
        KeyCode::Left | KeyCode::Char('a') => move_left(player),
        KeyCode::Right | KeyCode::Char('d') => move_right(player, map),
        _ => {}
    }
}

fn move_up(player: &mut Player){
    player.y = player.y.saturating_sub(1);
}

fn move_down(player: &mut Player, map: &Map){
    if player.y + 1 >= map.height{
        return;
    }

    player.y += 1;
}

fn move_left(player: &mut Player){
    player.x = player.x.saturating_sub(1);
}

fn move_right(player: &mut Player, map: &Map){
    if player.x + 1 >= map.width{
        return;
    }

    player.x += 1;
}