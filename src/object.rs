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

pub trait Entity : Object{
    fn move_cell(&mut self, direction: Direction, map: &Map) -> bool;

    fn check_mobility(&self, direction: Direction, map: &Map) -> bool;
}
