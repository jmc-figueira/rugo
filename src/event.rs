use tcod::input::*;
use tcod::console::*;
use object::{Direction, EntityManager};
use item::*;
use ui::{SciUI, SystemMessages};
use map::Map;

const MOVE_COST: u64 = 1;

pub enum Event{
    Look(u64),
    PickUp(u64),
    Drop(u64, u64),
    Move(u64, Direction, u64),
    Walk(u64, Direction, u64),
}

impl Event{
    pub fn execute(&self, event_queue: &mut EventQueue, entity_list: &mut EntityManager, items: &mut ItemManager, map: &Map, ui: &mut SystemMessages) -> u64{
        match *self{
            Event::Look(player_id) =>{
                if let Some(entity) = entity_list.get_entity_by_id(player_id){
                    let items_at = items.items_at(entity.get_coords().0, entity.get_coords().1);
                    match items_at.len(){
                        0 => {},
                        1 => ui.print(format!("There is a {} here.", items.get_item_by_id(items_at[0]).unwrap().get_name()).as_str()),
                        2 => ui.print(format!("There is a {} and a {} here.", items.get_item_by_id(items_at[0]).unwrap().get_name(), items.get_item_by_id(items_at[1]).unwrap().get_name()).as_str()),
                        _ => ui.print("There are several items here."),
                    }
                }
                0
            },
            Event::PickUp(player_id) =>{
                if let Some(player) = entity_list.get_entity_by_id(player_id){
                    let items_at = items.items_at(player.get_coords().0, player.get_coords().1);
                    match items_at.len(){
                        0 => {
                            ui.print("There is nothing to pick up here.");
                            return 0;
                        },
                        1 => {
                            ui.print(format!("You picked up the {}.", items.get_item_by_id(items_at[0]).unwrap().get_name()).as_str());
                            items.transfer(items_at[0], &mut player.as_player().unwrap().inventory);
                            return 1;
                        },
                        _ => {
                            return 1;
                        }
                    }
                }
                0
            },
            Event::Drop(player_id, item_id) => {
                0
            },
            Event::Move(id, dir, cost) => {
                if let Some(entity) = entity_list.get_entity_by_id(id){
                    if entity.move_cell(dir, map){
                        event_queue.push(Event::Look(entity.get_id()));
                        return cost;
                    }
                }
                0
            },
            Event::Walk(id, dir, cost) => {
                if let Some(entity) = entity_list.get_entity_by_id(id){
                    if entity.check_mobility(dir, map){
                        entity.move_cell(dir, map);
                        event_queue.push(Event::Walk(id, dir, cost));
                        return cost;
                    }
                }
                0
            }
        }
    }
}

pub trait EventQueue{
    fn push(&mut self, event: Event);

    fn poll(&mut self, root: &mut Root, ui: &mut SciUI, map: &Map, entity_list: &mut EntityManager, player_id: u64, items: &mut ItemManager) -> bool;
}

pub struct TurnBasedEventQueue{
    queue: Vec<Event>,
    turns: u64,
}

impl TurnBasedEventQueue{
    pub fn new() -> TurnBasedEventQueue{
        TurnBasedEventQueue{
            queue: Vec::new(),
            turns: 0
        }
    }

    pub fn handle_input(&mut self, root: &mut Root, ui: &mut SciUI, player_id: u64) -> bool{
        if let Some(key) = root.check_for_keypress(KEY_PRESSED){
            match key{
                Key{code: KeyCode::Escape, ..} => {
                    if ui.inv_visible{
                        ui.inv_visible = false;
                        return false;
                    }
                    true
                },
                Key{printable: 'p', ctrl: true, shift: true, ..} => {
                    let full_mesg_box = ui.show_all(root.width(), root.height());
                    blit(&full_mesg_box, (0, 0), (root.width(), root.height()), root, (0, 0), 1.0, 1.0);
                    root.flush();
                    root.wait_for_keypress(true);
                    root.clear();
                    false
                },
                Key{printable: 'i', ..} => {
                    ui.inv_visible = true;
                    false
                }
                Key{code, printable, shift: true, ..} => {
                    self.shift_commands(code, printable, player_id);
                    false
                },
                Key{code: KeyCode::NumPad8, ..} | Key{printable: 'w', ..} => {
                    self.push(Event::Move(player_id, Direction::N, MOVE_COST));
                    false
                },
                Key{code: KeyCode::NumPad2, ..} | Key{printable: 'x', ..} => {
                    self.push(Event::Move(player_id, Direction::S, MOVE_COST));
                    false
                },
                Key{code: KeyCode::NumPad6, ..} | Key{printable: 'd', ..} => {
                    self.push(Event::Move(player_id, Direction::E, MOVE_COST));
                    false
                },
                Key{code: KeyCode::NumPad4, ..} | Key{printable: 'a', ..} => {
                    self.push(Event::Move(player_id, Direction::W, MOVE_COST));
                    false
                },
                Key{code: KeyCode::NumPad7, ..} | Key{printable: 'q', ..} => {
                    self.push(Event::Move(player_id, Direction::NW, MOVE_COST));
                    false
                },
                Key{code: KeyCode::NumPad9, ..} | Key{printable: 'e', ..} => {
                    self.push(Event::Move(player_id, Direction::NE, MOVE_COST));
                    false
                },
                Key{code: KeyCode::NumPad3, ..} | Key{printable: 'c', ..} => {
                    self.push(Event::Move(player_id, Direction::SE, MOVE_COST));
                    false
                },
                Key{code: KeyCode::NumPad1, ..} | Key{printable: 'z', ..} => {
                    self.push(Event::Move(player_id, Direction::SW, MOVE_COST));
                    false
                },
                Key{printable: ',', ..} => {
                    self.push(Event::PickUp(player_id));
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

    fn shift_commands(&mut self, key: KeyCode, printable: char, player_id: u64) {
        if key == KeyCode::NumPad8 || printable == 'W' {
            self.push(Event::Walk(player_id, Direction::N, MOVE_COST));
        } else if key == KeyCode::NumPad2 || printable == 'X' {
            self.push(Event::Walk(player_id, Direction::S, MOVE_COST));
        } else if key == KeyCode::NumPad6 || printable == 'D' {
            self.push(Event::Walk(player_id, Direction::E, MOVE_COST));
        } else if key == KeyCode::NumPad4 || printable == 'A' {
            self.push(Event::Walk(player_id, Direction::W, MOVE_COST));
        } else if key == KeyCode::NumPad7 || printable == 'Q' {
            self.push(Event::Walk(player_id, Direction::NW, MOVE_COST));
        } else if key == KeyCode::NumPad9 || printable == 'E' {
            self.push(Event::Walk(player_id, Direction::NE, MOVE_COST));
        } else if key == KeyCode::NumPad3 || printable == 'C' {
            self.push(Event::Walk(player_id, Direction::SE, MOVE_COST));
        } else if key == KeyCode::NumPad1 || printable == 'Z' {
            self.push(Event::Walk(player_id, Direction::SW, MOVE_COST));
        }
    }

    pub fn current_turns(&self) -> u64{
        self.turns
    }
}

impl EventQueue for TurnBasedEventQueue{
    fn push(&mut self, event: Event){
        self.queue.push(event);
    }

    fn poll(&mut self, root: &mut Root, ui: &mut SciUI, map: &Map, entity_list: &mut EntityManager, player_id: u64, items: &mut ItemManager) -> bool{
        let next_event = self.queue.pop();

        match next_event{
            Some(event) => {
                self.turns += event.execute(self, entity_list, items, map, ui);
                false
            },
            None => {
                self.handle_input(root, ui, player_id)
            }
        }
    }
}
