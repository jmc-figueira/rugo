extern crate tcod;
extern crate rand;

mod object;
mod colors;
mod player;
mod tile;
mod map;
mod event;
mod ui;
mod stats;
mod monster;
mod dice;
mod item;

use colors::*;
use tcod::console::*;
use object::*;
use player::*;
use map::*;
use item::*;
use ui::*;
use dice::*;
use event::{EventQueue, TurnBasedEventQueue};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FPS: i32 = 60;

fn main(){
    let mut root = Root::initializer().renderer(Renderer::GLSL).font("consolas12x12_gs_tc.png", FontLayout::Tcod).font_type(FontType::Greyscale).size(SCREEN_WIDTH, SCREEN_HEIGHT).title(format!("Rugo {}", VERSION)).init();

    tcod::system::set_fps(FPS);

    let mut ui = SciUI::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let (map_builder, player_pos) = MapBuilder::new(SCREEN_WIDTH, SCREEN_HEIGHT).generate_cave();

    let mut map = map_builder.build();

    let mut entity_gen = IDManager::new();

    let mut item_gen = IDManager::new();

    let mut pobj = Player::new(&mut entity_gen, player_pos.0, player_pos.1, '@', DARK, PLAYER, 2f32);

    let mut starting_wep = Item::new(&mut item_gen, "Rusty Sword", player_pos.0, player_pos.1, '/', DARK, (255, 255, 255));

    let mut entities = EntityManager::new();

    let mut items = ItemManager::new();

    let player = entities.register(&mut pobj);

    let weapon = items.add(starting_wep);

    let mut event_queue = TurnBasedEventQueue::new();

    let mut world_console = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut quit = false;

    let mut hud_shift = false;
    let mut mesg_shift = false;

    ui.print("Welcome to The Arena...");
    ui.print("Do you have what it takes to survive?");

    while !root.window_closed() && !quit{
        world_console.clear();

        let player_e = entities.get_entity_by_id(player).unwrap().as_player().unwrap();

        ui.update_hud(player_e.stats.clone(), event_queue.current_turns());

        let player_coords = player_e.get_coords();

        let player_ls = player_e.get_light_source();

        entities.register(player_e);

        map.render(&mut world_console, player_ls, Vec::new(), &items, &entities);

        hud_shift = if player_coords.1 > ((SCREEN_HEIGHT - 1) - (SCREEN_HEIGHT / 3)){ false } else if player_coords.1 <= (SCREEN_HEIGHT / 3){ true } else{ hud_shift };

        mesg_shift = if player_coords.0 > ((SCREEN_WIDTH - 1) - (SCREEN_WIDTH / 3)){ false } else if player_coords.0 <= (SCREEN_WIDTH / 3){ true } else{ mesg_shift };



        if hud_shift{
            blit(ui.show_hud(), (0, 0), (ui.hud_width, ui.hud_height), &mut world_console, (0, SCREEN_HEIGHT - ui.hud_height), 1.0, 1.0);
        } else{
            blit(ui.show_hud(), (0, 0), (ui.hud_width, ui.hud_height), &mut world_console, (0, 0), 1.0, 1.0);
        }

        if mesg_shift{
            blit(ui.show_messages(), (0, 0), (ui.mesg_width, ui.mesg_height), &mut world_console, (SCREEN_WIDTH - ui.mesg_width, if hud_shift{ 0 } else{ ui.hud_height }), 1.0, 1.0);
        } else{
            blit(ui.show_messages(), (0, 0), (ui.mesg_width, ui.mesg_height), &mut world_console, (0, if hud_shift{ 0 } else{ ui.hud_height }), 1.0, 1.0);
        }

        blit(&world_console, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT), &mut root, (0, 0), 1.0, 1.0);

        root.flush();
        quit = event_queue.poll(&mut root, &mut ui, &map, &mut entities, player);
    }
}
