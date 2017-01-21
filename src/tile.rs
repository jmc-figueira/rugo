use tcod::colors::Color;

pub struct Tile{
    pub block: bool,
    pub graphic: char,
    pub color: Color,
}

impl Tile{
    pub fn new(wall: bool, graphic: char, color: Color) -> Tile{
        Tile{
            block: wall,
            graphic: graphic,
            color: color
        }
    }
}
