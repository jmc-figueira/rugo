use colors::ColorCell;

pub struct Tile{
    block: bool,
    pub graphic: char,
    pub color: ColorCell,
}

impl Tile{
    pub fn new(wall: bool, graphic: char, bg: (u8, u8, u8), fg: (u8, u8, u8)) -> Tile{
        Tile{
            block: wall,
            graphic: graphic,
            color: ColorCell::new(bg, fg)
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
            color: self.color.clone()
        }
    }
}
