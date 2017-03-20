extern crate tcod;
extern crate rand;

mod object;
mod colors;
mod player;
mod tile;
mod map;
mod event;

use colors::*;
use tcod::console::*;
use tcod::input::*;
use object::*;
use player::*;
use map::*;
use event::{Event, EventQueue, TurnBasedEventQueue};
use std::any::Any;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FPS: i32 = 60;

fn main(){
    let mut root = Root::initializer().renderer(Renderer::GLSL).font("consolas12x12_gs_tc.png", FontLayout::Tcod).font_type(FontType::Greyscale).size(SCREEN_WIDTH, SCREEN_HEIGHT).title(format!("Rugo {}", VERSION)).init();

    tcod::system::set_fps(FPS);

    let mut map = MapBuilder::new(SCREEN_WIDTH, SCREEN_HEIGHT).generate_cave().build();

    let mut entities = EntityManager::new();

    let player_pos = map.get_random_empty_tile();

    let mut id_gen = IDManager::new();

    let player = entities.register(&mut Player::new(&mut id_gen, player_pos.0, player_pos.1, '@', DARK, PLAYER, 2f32));

    let mut event_queue = TurnBasedEventQueue::new();

    let mut world_console = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut quit = false;

    while !root.window_closed() && !quit{
        event_queue.poll(&mut entities, &map);

        unsafe{
            let player_e = entities.getEntityById(player).unwrap() as &Any;
            map.render(&mut world_console, player_e);
            player_e.render(&mut world_console);
            entities.register(player_e);
        }

        blit(&mut world_console, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT), &mut root, (0, 0), 1.0, 1.0);
        root.flush();
        quit = handle_input(&mut root, &mut event_queue, player, &map);
    }
}

fn handle_input(root: &mut Root, event_queue: &mut EventQueue, player_id: u64, map: &Map) -> bool{
    if let Some(key) = root.check_for_keypress(KEY_PRESSED){
        match key{
            Key{code: KeyCode::Escape, ..} => true,
            Key{code, printable, shift: true, ..} => {
                shift_commands(code, printable, event_queue, player_id, map);
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

fn shift_commands(key: KeyCode, printable: char, event_queue: &mut EventQueue, player_id: u64, map: &Map){
    if key == KeyCode::NumPad8 || printable == 'W'{
        event_queue.push(Event::Walk(player_id, Direction::N));
    }
    else if key == KeyCode::NumPad2 || printable == 'X'{
        event_queue.push(Event::Walk(player_id, Direction::S));
    }
    else if key == KeyCode::NumPad6 || printable == 'D'{
        event_queue.push(Event::Walk(player_id, Direction::E));
    }
    else if key == KeyCode::NumPad4 || printable == 'A'{
        event_queue.push(Event::Walk(player_id, Direction::W));
    }
    else if key == KeyCode::NumPad7 || printable == 'Q'{
        event_queue.push(Event::Walk(player_id, Direction::NW));
    }
    else if key == KeyCode::NumPad9 || printable == 'E'{
        event_queue.push(Event::Walk(player_id, Direction::NE));
    }
    else if key == KeyCode::NumPad3 || printable == 'C'{
        event_queue.push(Event::Walk(player_id, Direction::SE));
    }
    else if key == KeyCode::NumPad1 || printable == 'Z'{
        event_queue.push(Event::Walk(player_id, Direction::SW));
    }
}
