use colors::ColorCell;
use colors::Rgb;

pub struct Tile{
    block: bool,
    explored: bool,
    pub graphic: char,
    pub color: ColorCell,
    pub absorption: f32,
}

impl Tile{
    pub fn new(wall: bool, graphic: char, bg: Rgb, fg: Rgb, absorption: f32) -> Tile{
        Tile{
            block: wall,
            explored: false,
            graphic: graphic,
            color: ColorCell::new(bg, fg),
            absorption: absorption
        }
    }

    pub fn is_blocked(&self) -> bool{
        self.block
    }

    pub fn is_explored(&self) -> bool{
        self.explored
    }

    pub fn toggle_explored(&mut self){
        self.explored = !self.explored;
    }
}

impl Clone for Tile{
    fn clone(&self) -> Tile{
        Tile{
            block: self.block,
            explored: self.explored,
            graphic: self.graphic,
            color: self.color.clone(),
            absorption: self.absorption
        }
    }
}
