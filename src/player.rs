use tcod::console::*;
use colors::ColorCell;
use object::*;
use map::*;

pub struct Player{
    x: i32,
    y: i32,
    graphic: char,
    color: ColorCell,
}

impl Player{
    pub fn new(x: i32, y: i32, graphic: char, bg: (u8, u8, u8), fg: (u8, u8, u8)) -> Player{
        Player{
            x: x,
            y: y,
            graphic: graphic,
            color: ColorCell::new(bg, fg)
        }
    }

    pub fn walk(&mut self, direction: Direction, map: &Map){
        while self.move_cell(direction, map){

        }
    }
}

impl Object for Player{
    fn render(&self, console: &mut Console){
        console.set_default_background(*self.color.background());
        console.set_default_foreground(*self.color.foreground());
        console.put_char(self.x, self.y, self.graphic, BackgroundFlag::None);
    }
}

impl Entity for Player{
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
