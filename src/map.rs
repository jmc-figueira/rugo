use tcod::console::*;
use colors::*;
use player::*;
use tile::*;
use rand;
use rand::Rng;
use std::f64;

pub struct Map{
    pub width: i32,
    pub height: i32,
    map: Vec<Tile>,
}

impl Map{
    fn new(width: i32, height: i32) -> Map{
        let mut map: Vec<Tile> = Vec::new();

        for _ in 0..(width * height){
            map.push(Tile::new(true, ' ', (0, 0, 0), (0, 0, 0), 1f32));
        }

        Map{
            width: width,
            height: height,
            map: map
        }
    }

    pub fn get_neighbours(&self, x: i32, y: i32) -> Vec<Tile>{
        let mut ret_val: Vec<Tile> = Vec::new();

        if y > 0{
            ret_val.push(self.get_tile(x, y - 1).clone());
        }
        if y < self.height - 1{
            ret_val.push(self.get_tile(x, y + 1).clone());

        }
        if x > 0{
            ret_val.push(self.get_tile(x - 1, y).clone());
            if y > 0{
                ret_val.push(self.get_tile(x - 1, y - 1).clone());
            }
            if y < self.height - 1{
                ret_val.push(self.get_tile(x - 1, y + 1).clone());
            }
        }
        if x < self.width - 1{
            ret_val.push(self.get_tile(x + 1, y).clone());
            if y > 0{
                ret_val.push(self.get_tile(x + 1, y - 1).clone());
            }
            if y < self.height - 1{
                ret_val.push(self.get_tile(x + 1, y + 1).clone());
            }
        }

        ret_val
    }

    pub fn get_tile(&self, x: i32, y: i32) -> &Tile{
        &self.map[(y * self.width + x) as usize]
    }

    pub fn change_tile(&mut self, x: i32, y: i32, tile: Tile){
        self.map[(y * self.width + x) as usize] = tile;
    }

    pub fn is_blocked(&self, x: i32, y: i32) -> bool{
        self.map[(y * self.width + x) as usize].is_blocked()
    }

    pub fn render(&self, console: &mut Console, player: &Player){
        let mut iteration = 0;
        let mut angle = 0f64;
        let mut curr_light_level = player.light_intensity;

        while angle < 2f64 * f64::consts::PI{
            let curr_x = player.x + ((iteration as f64) * angle.cos()).round() as i32;
            let curr_y = player.y + ((iteration as f64) * angle.sin()).round() as i32;
            if curr_x < 0 || curr_x >= self.width || curr_y < 0 || curr_y >= self.height{
                angle += f64::consts::PI / 1000f64;
                iteration = 0;
                curr_light_level = player.light_intensity;
                continue;
            }

            let cell = self.get_tile(curr_x, curr_y);

            let blended = cell.color.blend_light(&player.light, curr_light_level);
            console.set_default_background(*blended.background());
            console.set_default_foreground(*blended.foreground());
            console.put_char(curr_x, curr_y, cell.graphic, BackgroundFlag::None);

            curr_light_level -= cell.absorption;
            iteration += 1;
        }
    }
}

impl Clone for Map{
    fn clone(&self) -> Map{
        Map{
            width: self.width,
            height: self.height,
            map: self.map.clone()
        }
    }
}



pub struct MapBuilder{
    map: Map,
}

impl MapBuilder{
    pub fn new(width: i32, height: i32) -> MapBuilder{
        MapBuilder{
            map: Map::new(width, height)
        }
    }

    fn create_room(mut self, x: i32, y: i32, width: i32, height: i32) -> MapBuilder{
        if x >= 0 && x < self.map.width && (x + width) < self.map.width && y >= 0 && y < self.map.height && (y + height) < self.map.height{
            for i in (x + 1)..(x + width){
                for j in (y + 1)..(y + height){
                    self.map.change_tile(i, j, Tile::new(false, '_', (0, 0, 0), (255, 255, 255), 0.15));
                }
            }
            for i in x..(x + width + 1){
                self.map.change_tile(i, y, Tile::new(true, '#', (0, 0, 0), (255, 255, 255), 1f32));
                self.map.change_tile(i, (y + height), Tile::new(true, '#', (0, 0, 0), (255, 255, 255), 1f32));
            }

            for j in y..(y + height + 1){
                self.map.change_tile(x, j, Tile::new(true, '#', (0, 0, 0), (255, 255, 255), 1f32));
                self.map.change_tile((x + width), j, Tile::new(true, '#', (0, 0, 0), (255, 255, 255), 1f32));
            }
        }
        self
    }

    fn create_horizontal_corridor(mut self, x: i32, y: i32, width: i32) -> MapBuilder{
        if x >= 0 && x < self.map.width && (x + width) < self.map.width{
            for i in x..(x + width){
                self.map.change_tile(i, y, Tile::new(false, '.', (0, 0, 0), (127, 127, 127), 0.15));
            }
        }
        self
    }

    fn create_vertical_corridor(mut self, x: i32, y: i32, height: i32) -> MapBuilder{
        if y >= 0 && y < self.map.height && (y + height) < self.map.height{
            for j in y..(y + height){
                self.map.change_tile(x, j, Tile::new(false, '.', (0, 0, 0), (127, 127, 127), 0.15));
            }
        }
        self
    }

    pub fn load_from_file(self, file_name: &str) -> MapBuilder{

        self
    }

    pub fn generate_cave(mut self) -> Map{
        for i in 0..self.map.width{
            for j in 0..self.map.height{
                let wall_chance = rand::thread_rng().gen_range(0, 100);

                if wall_chance < 55{
                    self.map.change_tile(i, j, Tile::new(true, '#', (0, 0, 0), (255, 255, 255), 1f32));
                }
                else{
                    self.map.change_tile(i, j, Tile::new(false, '.', (0, 0, 0), (255, 255, 255), 0.15));
                }
            }
        }

        for _ in 0..5{
            let mut new_map = self.map.clone();
            for j in 0..self.map.height{
                for i in 0..self.map.width{
                    let neighbours = self.map.get_neighbours(i, j);
                    if neighbours.len() < 8{
                        new_map.change_tile(i, j, Tile::new(true, '#', (0, 0, 0), (255, 255, 255), 1f32));
                    }
                    else{
                        if MapBuilder::count_walls(&neighbours) < 5{
                            new_map.change_tile(i, j, Tile::new(true, '#', (0, 0, 0), (255, 255, 255), 1f32));
                        }
                        else{
                            new_map.change_tile(i, j, Tile::new(false, '.', (0, 0, 0), (255, 255, 255), 0.15));
                        }
                    }
                }
            }
            self.map = new_map;
        }

        self.map
    }

    fn count_walls(neighbours: &Vec<Tile>) -> u8{
        let mut acum = 0u8;
        for cell in neighbours.iter(){
            if cell.is_blocked(){
                acum += 1;
            }
        }
        acum
    }
}
