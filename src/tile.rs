use colors::ColorCell;
use colors::Rgb;

pub struct Tile{
    block: bool,
    pub graphic: char,
    pub color: ColorCell,
    pub absorption: f32,
}

impl Tile{
    pub fn new(wall: bool, graphic: char, bg: Rgb, fg: Rgb, absorption: f32) -> Tile{
        Tile{
            block: wall,
            graphic: graphic,
            color: ColorCell::new(bg, fg),
            absorption: absorption
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
            color: self.color.clone(),
            absorption: self.absorption
        }
    }
}
