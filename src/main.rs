extern crate tcod;
extern crate rand;

mod object;
mod player;
mod tile;
mod map;

use tcod::console::*;
use tcod::colors;
use tcod::input::*;
use object::*;
use player::*;
use map::*;


const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FPS: i32 = 60;

fn main(){
    let mut root = Root::initializer().font("consolas12x12_gs_tc.png", FontLayout::Tcod).font_type(FontType::Greyscale).size(SCREEN_WIDTH, SCREEN_HEIGHT).title("Rugo").init();

    tcod::system::set_fps(FPS);

    let mut player = Player::new(5, 5, '@', colors::WHITE);

    let map = MapBuilder::new(SCREEN_WIDTH, SCREEN_HEIGHT).generate_cave();

    let mut world_console = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    while !root.window_closed(){
        map.render(&mut world_console);
        player.render(&mut world_console);

        blit(&mut world_console, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT), &mut root, (0, 0), 1.0, 1.0);
        root.flush();
        handle_input(&mut root, &mut player, &map);
    }
}

fn handle_input(root: &mut Root, player: &mut Player, map: &Map){
    let key = root.wait_for_keypress(true);
    match key{
        Key{code: KeyCode::NumPad8, ..} => {
            player.move_cell(Direction::N, map);
        },
        Key{code: KeyCode::NumPad2, ..} => {
            player.move_cell(Direction::S, map);
        },
        Key{code: KeyCode::NumPad6, ..} => {
            player.move_cell(Direction::E, map);
        },
        Key{code: KeyCode::NumPad4, ..} => {
            player.move_cell(Direction::W, map);
        },
        Key{code: KeyCode::NumPad7, ..} => {
            player.move_cell(Direction::NW, map);
        },
        Key{code: KeyCode::NumPad9, ..} => {
            player.move_cell(Direction::NE, map);
        },
        Key{code: KeyCode::NumPad3, ..} => {
            player.move_cell(Direction::SE, map);
        },
        Key{code: KeyCode::NumPad1, ..} => {
            player.move_cell(Direction::SW, map);
        },
        _ => {},
    }
}
