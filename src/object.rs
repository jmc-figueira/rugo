use tcod::console::*;
use map::*;
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

pub trait Entity : Object{
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
        self.entities.insert(entity.get_id(), entity);
        entity.get_id()
    }

    pub fn getEntityById(&mut self, id: u64) -> Option<&'root mut Entity>{
        self.entities.remove(&id)
    }
}
