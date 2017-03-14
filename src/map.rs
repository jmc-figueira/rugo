use tcod::console::*;
use colors::*;
use player::*;
use tile::*;
use rand;
use rand::Rng;
use std::f64;
use std::f32;

pub struct Map{
    pub width: i32,
    pub height: i32,
    map: Vec<Tile>,
    memory: Vec<(i32, i32)>,
}

impl Map{
    fn new(width: i32, height: i32) -> Map{
        let mut map: Vec<Tile> = Vec::new();

        for _ in 0..(width * height){
            map.push(Tile::new(true, ' ', DARK, DARK, 1f32));
        }

        Map{
            width: width,
            height: height,
            map: map,
            memory: Vec::new()
        }
    }

    pub fn get_random_empty_tile(&self) -> (i32, i32){
        loop{
            let ret_x = rand::thread_rng().gen_range(0, self.width);
            let ret_y = rand::thread_rng().gen_range(0, self.height);

            if !self.is_blocked(ret_x, ret_y){
                return (ret_x, ret_y);
            }
        }
    }

    pub fn flood_fill(&self, start_x: i32, start_y: i32) -> Vec<(i32, i32)>{
        let mut ret_val: Vec<(i32, i32)> = Vec::new();
        let mut visited: Vec<(i32, i32)> = Vec::new();

        visited.push((start_x, start_y));

        while !visited.is_empty(){

            let current = visited.pop().unwrap();

            ret_val.push(current);

            if self.is_blocked(current.0, current.1){
                continue;
            }

            let neighbours = self.get_neighbouring_empty(current.0, current.1);

            for coords in neighbours.iter(){
                if !ret_val.iter().any(|c| c.0 == coords.0 && c.1 == coords.1) && !visited.iter().any(|c| c.0 == coords.0 && c.1 == coords.1){
                    visited.push(*coords);
                }
            }
        }

        ret_val
    }

    fn get_neighbouring_empty(&self, x: i32, y: i32) -> Vec<(i32, i32)>{
        let mut ret_val: Vec<(i32, i32)> = Vec::new();

        if y > 0{
            if !self.is_blocked(x, y - 1){
                ret_val.push((x, y - 1));
            }
        }
        if y < self.height - 1{
            if !self.is_blocked(x, y + 1){
                ret_val.push((x, y + 1));
            }
        }
        if x > 0{
            if !self.is_blocked(x - 1, y){
                ret_val.push((x - 1, y));
            }
            if y > 0{
                if !self.is_blocked(x - 1, y - 1){
                    ret_val.push((x - 1, y - 1));
                }
            }
            if y < self.height - 1{
                if !self.is_blocked(x - 1, y + 1){
                    ret_val.push((x - 1, y + 1));
                }
            }
        }
        if x < self.width - 1{
            if !self.is_blocked(x + 1, y){
                ret_val.push((x + 1, y));
            }
            if y > 0{
                if !self.is_blocked(x + 1, y - 1){
                    ret_val.push((x + 1, y - 1));
                }
            }
            if y < self.height - 1{
                if !self.is_blocked(x + 1, y + 1){
                    ret_val.push((x + 1, y + 1));
                }
            }
        }

        ret_val
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

    pub fn render(&mut self, console: &mut Console, player: &Player){
        let memory_color = ColorCell::new(DARK, MEMORY);
        for coords in self.memory.iter(){
            let cell = self.get_tile(coords.0 as i32, coords.1 as i32);

            console.set_default_background(*memory_color.background());
            console.set_default_foreground(*memory_color.foreground());
            console.put_char(coords.0 as i32, coords.1 as i32, cell.graphic, BackgroundFlag::None);
        }

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

            if !self.memory.iter().any(|c| c.0 == curr_x && c.1 == curr_y) && curr_light_level > 0f32{
                self.memory.push((curr_x, curr_y));
            }

            let cell = self.get_tile(curr_x, curr_y);

            if curr_light_level > 0f32{
                let blended = cell.color.blend_light(&player.light, curr_light_level);
                console.set_default_background(*blended.background());
                console.set_default_foreground(*blended.foreground());
                console.put_char(curr_x, curr_y, cell.graphic, BackgroundFlag::None);
            }

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
            map: self.map.clone(),
            memory: self.memory.clone()
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
                    self.map.change_tile(i, j, Tile::new(false, '_', DARK, (255, 255, 255), 0.15));
                }
            }
            for i in x..(x + width + 1){
                self.map.change_tile(i, y, Tile::new(true, '#', DARK, (255, 255, 255), 1f32));
                self.map.change_tile(i, (y + height), Tile::new(true, '#', DARK, (255, 255, 255), 1f32));
            }

            for j in y..(y + height + 1){
                self.map.change_tile(x, j, Tile::new(true, '#', DARK, (255, 255, 255), 1f32));
                self.map.change_tile((x + width), j, Tile::new(true, '#', DARK, (255, 255, 255), 1f32));
            }
        }
        self
    }

    fn create_horizontal_corridor(mut self, x: i32, y: i32, width: i32) -> MapBuilder{
        if x >= 0 && x < self.map.width && (x + width) < self.map.width{
            for i in x..(x + width){
                self.map.change_tile(i, y, Tile::new(false, '.', DARK, (127, 127, 127), 0.15));
            }
        }
        self
    }

    fn create_vertical_corridor(mut self, x: i32, y: i32, height: i32) -> MapBuilder{
        if y >= 0 && y < self.map.height && (y + height) < self.map.height{
            for j in y..(y + height){
                self.map.change_tile(x, j, Tile::new(false, '.', DARK, (127, 127, 127), 0.15));
            }
        }
        self
    }

    pub fn load_from_file(self, file_name: &str) -> MapBuilder{

        self
    }

    pub fn generate_cave(mut self) -> Map{
        loop{
            let mut tmp_map = self.map.clone();

            for i in 0..self.map.width{
                for j in 0..self.map.height{
                    let wall_chance = rand::thread_rng().gen_range(0, 100);

                    if wall_chance < 55{
                        tmp_map.change_tile(i, j, Tile::new(true, '#', DARK, CAVE_WALL, f32::MAX));
                    }
                    else{
                        tmp_map.change_tile(i, j, Tile::new(false, '.', DARK, CAVE_FLOOR, 0.15));
                    }
                }
            }

            for _ in 0..5{
                let mut new_map = tmp_map.clone();
                for j in 0..self.map.height{
                    for i in 0..self.map.width{
                        let neighbours = tmp_map.get_neighbours(i, j);
                        if neighbours.len() < 8{
                            new_map.change_tile(i, j, Tile::new(true, '#', DARK, CAVE_WALL, f32::MAX));
                        }
                        else{
                            if MapBuilder::count_walls(&neighbours) < 5{
                                new_map.change_tile(i, j, Tile::new(true, '#', DARK, CAVE_WALL, f32::MAX));
                            }
                            else{
                                new_map.change_tile(i, j, Tile::new(false, '.', DARK, CAVE_FLOOR, 0.15));
                            }
                        }
                    }
                }
                tmp_map = new_map;
            }
            let test_coords = tmp_map.get_random_empty_tile();
            let open_area = tmp_map.flood_fill(test_coords.0, test_coords.1);

            if open_area.len() as f64 >= 0.4 * tmp_map.width as f64 * tmp_map.height as f64{
                self.map = tmp_map;

                for j in 0..self.map.height{
                    for i in 0..self.map.width{
                        if !self.map.is_blocked(i, j) && !open_area.iter().any(|c| c.0 ==i && c.1 == j){
                            self.map.change_tile(i, j, Tile::new(true, '#', DARK, CAVE_WALL, f32::MAX));
                        }
                    }
                }

                break;
            }
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
