use object::{Direction, Entity};
use map::Map;

pub enum Event{
    Move(u64, Direction),
    Walk(u64, Direction),
}

impl Event{
    pub fn execute(&self, event_queue: &mut EventQueue, entity_list: Vec<&mut Entity>, map: &Map) -> bool{
        match *self{
            Event::Move(id, dir) => {
                match Event::findEntityById(entity_list, id){
                    Some(entity) => {
                        entity.move_cell(dir, map);
                        true
                    },
                    None => false
                }
            },
            Event::Walk(id, dir) => {
                match Event::findEntityById(entity_list, id){
                    Some(entity) => {
                        if entity.check_mobility(dir, map){
                            entity.move_cell(dir, map);
                            event_queue.push(Event::Walk(id, dir));
                            return true;
                        }
                        false
                    },
                    None => false
                }
            }
        }
    }

    fn findEntityById(entity_list: Vec<&mut Entity>, id: u64) -> Option<&mut Entity>{
        for entity in entity_list{
            if entity.get_id() == id{
                return Some(entity);
            }
        }
        None
    }
}

pub trait EventQueue{
    fn push(&mut self, event: Event);

    fn poll(&mut self, entity_list: Vec<&mut Entity>, map: &Map) -> bool;
}

pub struct TurnBasedEventQueue{
    queue: Vec<Event>,
}

impl TurnBasedEventQueue{
    pub fn new() -> TurnBasedEventQueue{
        TurnBasedEventQueue{
            queue: Vec::new()
        }
    }
}

impl EventQueue for TurnBasedEventQueue{
    fn push(&mut self, event: Event){
        self.queue.push(event);
    }

    fn poll(&mut self, entity_list: Vec<&mut Entity>, map: &Map) -> bool{
        let next_event = self.queue.pop();

        match next_event{
            Some(event) => {
                event.execute(self, entity_list, map)
            },
            None => false
        }
    }
}
