use crossterm::event::{Event, KeyCode, read};

use crate::core::{entities::Player, map::{Map, Tile}};

enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub fn get_input() -> Option<KeyCode> {
    if let Ok(Event::Key(key_event)) = read() {
        return Some(key_event.code);
    }
    None
}

pub fn handle_move(player: &mut Player, map: &Map, key: KeyCode) {
    match key {
        KeyCode::Up | KeyCode::Char('w') => move_up(player, map),
        KeyCode::Down | KeyCode::Char('s') => move_down(player, map),
        KeyCode::Left | KeyCode::Char('a') => move_left(player, map),
        KeyCode::Right | KeyCode::Char('d') => move_right(player, map),
        _ => {}
    }
}

fn move_restrictions(player: &Player, map: &Map, move_direction: Direction) -> bool{

    let target_position = match move_direction {
        Direction::Up =>{
            (player.x, player.y.saturating_sub(1))
        }
        Direction::Down =>{
            if player.y + 1 >= map.height{
                return true;
            } 
            
            (player.x, player.y+1)
        }
        Direction::Left =>{
            (player.x.saturating_sub(1), player.y)
        }
        Direction::Right =>{
            if player.x + 1 >= map.width{
                return true;
            }
            
            (player.x+1, player.y)
        }  
    };

    let (target_x, target_y) = target_position;

    if map.get_tile(target_x, target_y) == Tile::Wall{
        return true;        
    }

    false
}

fn move_up(player: &mut Player, map: &Map){
    
    if move_restrictions(player, map, Direction::Up) {return};
    
    player.y = player.y.saturating_sub(1);
}

fn move_down(player: &mut Player, map: &Map){
    
    if move_restrictions(player, map, Direction::Down) {return};

    player.y += 1;
}

fn move_left(player: &mut Player, map: &Map){
    
    if move_restrictions(player, map, Direction::Left) {return};
    
    player.x = player.x.saturating_sub(1);
}

fn move_right(player: &mut Player, map: &Map){

    if move_restrictions(player, map, Direction::Right) {return};

    player.x += 1;
}