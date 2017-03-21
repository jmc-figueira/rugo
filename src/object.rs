use tcod::console::*;
use map::*;
use player::Player;
use std::collections::HashMap;

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

    fn as_player(&mut self) -> Option<&mut Player>;

    fn move_cell(&mut self, direction: Direction, map: &Map) -> bool;

    fn check_mobility(&self, direction: Direction, map: &Map) -> bool;
}

pub struct EntityManager<'root>{
    entities: HashMap<u64, &'root mut Entity>,
}

impl<'root> EntityManager<'root>{
    pub fn new() -> EntityManager<'root>{
        EntityManager{
            entities: HashMap::new()
        }
    }

    pub fn register(&mut self, entity: &'root mut Entity) -> u64{
        let ret_val = entity.get_id();
        self.entities.insert(entity.get_id(), entity);
        ret_val
    }

    pub fn get_entity_by_id(&mut self, id: u64) -> Option<&'root mut Entity>{
        self.entities.remove(&id)
    }
}
