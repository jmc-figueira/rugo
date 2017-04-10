use tcod::console::*;
use tcod::colors::Color;
use colors::*;
use object::*;
use map::*;
use stats::*;

pub struct Player{
    id: u64,
    pub x: i32,
    pub y: i32,
    graphic: char,
    color: ColorCell,
    pub light: Light,
    pub stats: Stats,
}

impl Player{
    pub fn new(id_gen: &mut IDManager, x: i32, y: i32, graphic: char, bg: Rgb, fg: Rgb, light_intensity: f32) -> Player{
        let color_cell = ColorCell::new(bg, fg);
        Player{
            id: id_gen.next_id(),
            x: x,
            y: y,
            graphic: graphic,
            color: ColorCell::new(bg, fg),
            light: (*color_cell.foreground(), light_intensity),
            stats: Stats::new()
        }
    }

    pub fn get_light_source(&self) -> LightSource{
        (self.x, self.y, self.light)
    }
}

impl Object for Player{
    fn render(&self, console: &mut Console){
        console.put_char_ex(self.x, self.y, self.graphic, *self.color.foreground(), *self.color.background());
    }

    fn get_graphic(&self) -> char{
        self.graphic
    }

    fn get_coords(&self) -> (i32, i32){
        (self.x, self.y)
    }

    fn get_color(&self) -> ColorCell{
        self.color.clone()
    }
}

impl Entity for Player{
    fn get_id(&self) -> u64{
        self.id
    }

    fn as_player(&mut self) -> Option<&mut Player>{
        Some(self)
    }

    fn move_cell(&mut self, direction: Direction, map: &Map) -> bool{
        if self.check_mobility(direction, map){
            match direction{
                Direction::N => self.y -= 1,
                Direction::S => self.y += 1,
                Direction::E => self.x += 1,
                Direction::W => self.x -= 1,
                Direction::NW => {
                    self.x -= 1;
                    self.y -= 1;
                },
                Direction::NE => {
                    self.x += 1;
                    self.y -= 1;
                },
                Direction::SW => {
                    self.x -= 1;
                    self.y += 1;
                },
                Direction::SE => {
                    self.x += 1;
                    self.y += 1;
                },
            }
            return true;
        }
        false
    }

    fn check_mobility(&self, direction: Direction, map: &Map) -> bool{
        let mut check_x = self.x;
        let mut check_y = self.y;
        match direction{
            Direction::N => check_y -= 1,
            Direction::S => check_y += 1,
            Direction::E => check_x += 1,
            Direction::W => check_x -= 1,
            Direction::NW => {
                check_x -= 1;
                check_y -= 1;
            },
            Direction::NE => {
                check_x += 1;
                check_y -= 1;
            },
            Direction::SW => {
                check_x -= 1;
                check_y += 1;
            },
            Direction::SE => {
                check_x += 1;
                check_y += 1;
            },
        }

        check_x >= 0 && check_x < map.width && check_y >= 0 && check_y < map.height && !map.is_blocked(check_x, check_y)
    }
}

impl StatDriven for Player{
    fn get_stats(&self) -> &Stats{
        &self.stats
    }
}
