#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tile {
    Empty,
    Wall,
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![Tile::Empty; width * height];
        Map {
            width,
            height,
            tiles,
        }
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }
}
