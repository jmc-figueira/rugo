use object::{Direction, EntityManager};
use map::Map;

const MOVE_COST: u64 = 1;

pub enum Event{
    Move(u64, Direction),
    Walk(u64, Direction),
}

impl Event{
    pub fn execute(&self, event_queue: &mut EventQueue, entity_list: &mut EntityManager, map: &Map) -> bool{
        match *self{
            Event::Move(id, dir) => {
                match entity_list.get_entity_by_id(id){
                    Some(entity) => {
                        entity.move_cell(dir, map);
                        entity_list.register(entity);
                        true
                    },
                    None => false
                }
            },
            Event::Walk(id, dir) => {
                match entity_list.get_entity_by_id(id){
                    Some(entity) => {
                        if entity.check_mobility(dir, map){
                            entity.move_cell(dir, map);
                            entity_list.register(entity);
                            event_queue.push(Event::Walk(id, dir));
                            return true;
                        }
                        entity_list.register(entity);
                        false
                    },
                    None => false
                }
            }
        }
    }
}

pub trait EventQueue{
    fn push(&mut self, event: Event);

    fn poll(&mut self, entity_list: &mut EntityManager, map: &Map) -> bool;
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

    fn handle_input(root: &mut Root, ui: &mut SciUI, event_queue: &mut EventQueue, player_id: u64) -> bool{
        if let Some(key) = root.check_for_keypress(KEY_PRESSED){
            match key{
                Key{code: KeyCode::Escape, ..} => true,
                Key{printable: 'p', ctrl: true, shift: true, ..} => {
                    let full_mesg_box = ui.show_all(SCREEN_WIDTH, SCREEN_HEIGHT);
                    blit(&full_mesg_box, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT), root, (0, 0), 1.0, 1.0);
                    root.flush();
                    root.wait_for_keypress(true);
                    root.clear();
                    false
                },
                Key{code, printable, shift: true, ..} => {
                    shift_commands(code, printable, event_queue, player_id);
                    false
                },
                Key{code: KeyCode::NumPad8, ..} | Key{printable: 'w', ..} => {
                    event_queue.push(Event::Move(player_id, Direction::N));
                    false
                },
                Key{code: KeyCode::NumPad2, ..} | Key{printable: 'x', ..} => {
                    event_queue.push(Event::Move(player_id, Direction::S));
                    false
                },
                Key{code: KeyCode::NumPad6, ..} | Key{printable: 'd', ..} => {
                    event_queue.push(Event::Move(player_id, Direction::E));
                    false
                },
                Key{code: KeyCode::NumPad4, ..} | Key{printable: 'a', ..} => {
                    event_queue.push(Event::Move(player_id, Direction::W));
                    false
                },
                Key{code: KeyCode::NumPad7, ..} | Key{printable: 'q', ..} => {
                    event_queue.push(Event::Move(player_id, Direction::NW));
                    false
                },
                Key{code: KeyCode::NumPad9, ..} | Key{printable: 'e', ..} => {
                    event_queue.push(Event::Move(player_id, Direction::NE));
                    false
                },
                Key{code: KeyCode::NumPad3, ..} | Key{printable: 'c', ..} => {
                    event_queue.push(Event::Move(player_id, Direction::SE));
                    false
                },
                Key{code: KeyCode::NumPad1, ..} | Key{printable: 'z', ..} => {
                    event_queue.push(Event::Move(player_id, Direction::SW));
                    false
                },
                _ => {
                    false
                },
            }
        } else{
            false
        }
    }

    fn shift_commands(key: KeyCode, printable: char, event_queue: &mut EventQueue, player_id: u64) {
        if key == KeyCode::NumPad8 || printable == 'W' {
            event_queue.push(Event::Walk(player_id, Direction::N));
        } else if key == KeyCode::NumPad2 || printable == 'X' {
            event_queue.push(Event::Walk(player_id, Direction::S));
        } else if key == KeyCode::NumPad6 || printable == 'D' {
            event_queue.push(Event::Walk(player_id, Direction::E));
        } else if key == KeyCode::NumPad4 || printable == 'A' {
            event_queue.push(Event::Walk(player_id, Direction::W));
        } else if key == KeyCode::NumPad7 || printable == 'Q' {
            event_queue.push(Event::Walk(player_id, Direction::NW));
        } else if key == KeyCode::NumPad9 || printable == 'E' {
            event_queue.push(Event::Walk(player_id, Direction::NE));
        } else if key == KeyCode::NumPad3 || printable == 'C' {
            event_queue.push(Event::Walk(player_id, Direction::SE));
        } else if key == KeyCode::NumPad1 || printable == 'Z' {
            event_queue.push(Event::Walk(player_id, Direction::SW));
        }
    }
}

impl EventQueue for TurnBasedEventQueue{
    fn push(&mut self, event: Event){
        self.queue.push(event);
    }

    fn poll(&mut self, entity_list: &mut EntityManager, map: &Map) -> bool{
        let next_event = self.queue.pop();

        match next_event{
            Some(event) => {
                event.execute(self, entity_list, map)
            },
            None => false
        }
    }
}
