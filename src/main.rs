extern crate tcod;
extern crate rand;

mod object;
mod colors;
mod player;
mod tile;
mod map;

use colors::*;
use tcod::console::*;
use tcod::input::*;
use object::*;
use player::*;
use map::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FPS: i32 = 60;

fn main(){
    let mut root = Root::initializer().renderer(Renderer::GLSL).font("consolas12x12_gs_tc.png", FontLayout::Tcod).font_type(FontType::Greyscale).size(SCREEN_WIDTH, SCREEN_HEIGHT).title(format!("Rugo {}", VERSION)).init();

    tcod::system::set_fps(FPS);

    let mut map = MapBuilder::new(SCREEN_WIDTH, SCREEN_HEIGHT).generate_cave();

    let player_pos = map.get_random_empty_tile();

    let mut player = Player::new(player_pos.0, player_pos.1, '@', DARK, PLAYER, 2f32);

    let mut world_console = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut quit = false;

    while !root.window_closed() && !quit{
        map.render(&mut world_console, &player);
        player.render(&mut world_console);

        blit(&mut world_console, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT), &mut root, (0, 0), 1.0, 1.0);
        root.flush();
        quit = handle_input(&mut root, &mut player, &map);
    }
}

fn handle_input(root: &mut Root, player: &mut Player, map: &Map) -> bool{
    let key = root.wait_for_keypress(true);
    match key{
        Key{code: KeyCode::Escape, ..} => return true,
        Key{code, printable, shift: true, ..} => {
            shift_commands(code, printable, player, map);
            return false;
        },
        Key{code: KeyCode::NumPad8, ..} | Key{printable: 'w', ..} => {
            player.move_cell(Direction::N, map);
            return false;
        },
        Key{code: KeyCode::NumPad2, ..} | Key{printable: 'x', ..} => {
            player.move_cell(Direction::S, map);
            return false;
        },
        Key{code: KeyCode::NumPad6, ..} | Key{printable: 'd', ..} => {
            player.move_cell(Direction::E, map);
            return false;
        },
        Key{code: KeyCode::NumPad4, ..} | Key{printable: 'a', ..} => {
            player.move_cell(Direction::W, map);
            return false;
        },
        Key{code: KeyCode::NumPad7, ..} | Key{printable: 'q', ..} => {
            player.move_cell(Direction::NW, map);
            return false;
        },
        Key{code: KeyCode::NumPad9, ..} | Key{printable: 'e', ..} => {
            player.move_cell(Direction::NE, map);
            return false;
        },
        Key{code: KeyCode::NumPad3, ..} | Key{printable: 'c', ..} => {
            player.move_cell(Direction::SE, map);
            return false;
        },
        Key{code: KeyCode::NumPad1, ..} | Key{printable: 'z', ..} => {
            player.move_cell(Direction::SW, map);
            return false;
        },
        _ => {
            return false;
        },
    }
}

fn shift_commands(key: KeyCode, printable: char, player: &mut Player, map: &Map){
    if key == KeyCode::NumPad8 || printable == 'W'{
        player.walk(Direction::N, map);
    }
    else if key == KeyCode::NumPad2 || printable == 'X'{
        player.walk(Direction::S, map);
    }
    else if key == KeyCode::NumPad6 || printable == 'D'{
        player.walk(Direction::E, map);
    }
    else if key == KeyCode::NumPad4 || printable == 'A'{
        player.walk(Direction::W, map);
    }
    else if key == KeyCode::NumPad7 || printable == 'Q'{
        player.walk(Direction::NW, map);
    }
    else if key == KeyCode::NumPad9 || printable == 'E'{
        player.walk(Direction::NE, map);
    }
    else if key == KeyCode::NumPad3 || printable == 'C'{
        player.walk(Direction::SE, map);
    }
    else if key == KeyCode::NumPad1 || printable == 'Z'{
        player.walk(Direction::SW, map);
    }
}
