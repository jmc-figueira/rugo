use tcod::console::*;
use map::*;
use colors::ColorCell;
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

    fn get_graphic(&self) -> char;

    fn get_coords(&self) -> (i32, i32);

    fn get_color(&self) -> ColorCell;
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

pub struct EntityManager{
    entities: HashMap<u64, Box<Entity>>,
}

impl EntityManager{
    pub fn new() -> EntityManager{
        EntityManager{
            entities: HashMap::new()
        }
    }

    pub fn register(&mut self, entity: Box<Entity>) -> u64{
        let ret_val = entity.get_id();
        self.entities.insert(ret_val, entity);
        ret_val
    }

    pub fn get_entity_by_id(&mut self, id: u64) -> Option<&mut Box<Entity>>{
        self.entities.get_mut(&id)
    }

    pub fn get_entity_at(&self, x: i32, y: i32) -> Option<&Box<Entity>>{
        for entity in self.entities.values(){
            if entity.get_coords().0 == x && entity.get_coords().1 == y{
                return Some(entity);
            }
        }
        None
    }
}
