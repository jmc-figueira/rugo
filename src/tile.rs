use tcod::colors::Color;

pub struct Tile{
    block: bool,
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

    pub fn is_blocked(&self) -> bool{
        self.block
    }
}

impl Clone for Tile{
    fn clone(&self) -> Tile{
        Tile{
            block: self.block,
            graphic: self.graphic,
            color: self.color
        }
    }
}
