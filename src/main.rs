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

use colors::*;
use tcod::console::*;
use tcod::input::*;
use object::*;
use player::*;
use map::*;
use ui::*;
use dice::*;
use event::{Event, EventQueue, TurnBasedEventQueue};

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

    let mut id_gen = IDManager::new();

    let mut pobj = Player::new(&mut id_gen, player_pos.0, player_pos.1, '@', DARK, PLAYER, 2f32);

    let mut entities = EntityManager::new();

    let player = entities.register(&mut pobj);

    let mut event_queue = TurnBasedEventQueue::new();

    let mut world_console = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut quit = false;

    let mut hud_shift = false;
    let mut mesg_shift = false;

    ui.print("Welcome to The Arena...");

    while !root.window_closed() && !quit{
        world_console.clear();

        event_queue.poll(&mut entities, &map);

        let player_e = entities.get_entity_by_id(player).unwrap().as_player().unwrap();

        ui.update_hud(player_e.stats.clone());

        map.render(&mut world_console, player_e);
        player_e.render(&mut world_console);

        hud_shift = if player_e.y > ((SCREEN_HEIGHT - 1) - (SCREEN_HEIGHT / 3)){ false } else if player_e.y <= (SCREEN_HEIGHT / 3){ true } else{ hud_shift };

        mesg_shift = if player_e.x > ((SCREEN_WIDTH - 1) - (SCREEN_WIDTH / 3)){ false } else if player_e.x <= (SCREEN_WIDTH / 3){ true } else{ mesg_shift };

        entities.register(player_e);

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
        quit = handle_input(&mut root, &mut ui, &mut event_queue, player);
    }
}
