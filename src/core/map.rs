#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tile {
    Empty,
    Wall,
    Water,
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

    pub fn get_tile(&self, x: usize, y: usize) -> Tile {
        let idx = self.get_index(x, y);
        self.tiles[idx]
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        let idx: usize = self.get_index(x, y);
        self.tiles[idx] = tile;
    }
}
