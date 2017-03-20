use tcod::console::*;
use map::*;

#[derive(Copy, Clone)]
pub enum Direction{
    N,
    S,
    E,
    W,
    NW,
    NE,
    SW,
    SE,
}

pub trait Object{
    fn render(&self, console: &mut Console);
}

pub struct IDManager{
    current: u64,
}

impl IDManager{
    pub fn new() -> IDManager{
        IDManager{
            current: 0
        }
    }

    pub fn next_id(&mut self) -> u64{
        let ret_val = self.current;
        self.current += 1;
        ret_val
    }
}

pub trait Entity : Object{
    fn get_id(&self) -> u64;

    fn move_cell(&mut self, direction: Direction, map: &Map) -> bool;

    fn check_mobility(&self, direction: Direction, map: &Map) -> bool;
}
