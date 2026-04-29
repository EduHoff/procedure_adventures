use clearscreen::clear;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;

use crate::core::entities::Player;
use crate::core::map::Map;
use crate::core::map::Tile;

pub fn render(map: &Map, player: &Player) {
    clear().expect("Error: clear failed");

    for y in 0..map.height {
        for x in 0..map.width {
            if x == player.x && y == player.y {
                print!("@ ");
            } else {
                let idx = map.get_index(x, y);
                match map.tiles[idx] {
                    Tile::Empty => print!(". "),
                    Tile::Wall => print!("# "),
                }
            }
        }
        print!("\r\n")
    }
}

pub struct RawModeGuard;

impl Default for RawModeGuard {
     fn default() -> Self {
         Self::new()
     }
}

impl RawModeGuard {  
    pub fn new() -> Self {
        enable_raw_mode().expect("Error activating Raw Mode.");
        RawModeGuard
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}
