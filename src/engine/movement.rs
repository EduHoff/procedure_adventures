use crossterm::event::{Event, KeyCode, read};

use crate::core::entities::Player;

pub fn get_input() -> Option<KeyCode> {
    if let Ok(Event::Key(key_event)) = read() {
        return Some(key_event.code);
    }
    None
}

pub fn handle_move(player: &mut Player, key: KeyCode) {
    match key {
        KeyCode::Up | KeyCode::Char('w') => player.y = player.y.saturating_sub(1),
        KeyCode::Down | KeyCode::Char('s') => player.y += 1,
        KeyCode::Left | KeyCode::Char('a') => player.x = player.x.saturating_sub(1),
        KeyCode::Right | KeyCode::Char('d') => player.x += 1,
        _ => {}
    }
}
