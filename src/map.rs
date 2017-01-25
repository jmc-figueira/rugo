use tcod::console::*;
use tcod::colors;
use tile::*;
use rand;
use rand::Rng;

pub struct Map{
    pub width: i32,
    pub height: i32,
    map: Vec<Tile>,
}

impl Map{
    fn new(width: i32, height: i32) -> Map{
        let mut map: Vec<Tile> = Vec::new();

        for _ in 0..(width * height){
            map.push(Tile::new(true, ' ', colors::BLACK));
        }

        Map{
            width: width,
            height: height,
            map: map
        }
    }

    pub fn get_neighbours(&self, x: i32, y: i32) -> Vec<Tile>{
        let mut retVal: Vec<Tile> = Vec::new();

        if y > 0{
            retVal.push(self.get_tile(x, y - 1).clone());
        }
        if y < self.height - 1{
            retVal.push(self.get_tile(x, y + 1).clone());

        }
        if x > 0{
            retVal.push(self.get_tile(x - 1, y).clone());
            if y > 0{
                retVal.push(self.get_tile(x - 1, y - 1).clone());
            }
            if y < self.height - 1{
                retVal.push(self.get_tile(x - 1, y + 1).clone());
            }
        }
        if x < self.width - 1{
            retVal.push(self.get_tile(x + 1, y).clone());
            if y > 0{
                retVal.push(self.get_tile(x + 1, y - 1).clone());
            }
            if y < self.height - 1{
                retVal.push(self.get_tile(x + 1, y + 1).clone());
            }
        }

        retVal
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

    pub fn render(&self, console: &mut Console){
        for (i, cell) in self.map.iter().enumerate(){
            console.set_default_foreground(cell.color);
            console.put_char((i as i32) % self.width, (i as i32) / self.width, cell.graphic, BackgroundFlag::None);
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
                    self.map.change_tile(i, j, Tile::new(false, '_', colors::WHITE));
                }
            }
            for i in x..(x + width + 1){
                self.map.change_tile(i, y, Tile::new(true, '#', colors::WHITE));
                self.map.change_tile(i, (y + height), Tile::new(true, '#', colors::WHITE));
            }

            for j in y..(y + height + 1){
                self.map.change_tile(x, j, Tile::new(true, '#', colors::WHITE));
                self.map.change_tile((x + width), j, Tile::new(true, '#', colors::WHITE));
            }
        }
        self
    }

    fn create_horizontal_corridor(mut self, x: i32, y: i32, width: i32) -> MapBuilder{
        if x >= 0 && x < self.map.width && (x + width) < self.map.width{
            for i in x..(x + width){
                self.map.change_tile(i, y, Tile::new(false, '.', colors::GREY));
            }
        }
        self
    }

    fn create_vertical_corridor(mut self, x: i32, y: i32, height: i32) -> MapBuilder{
        if y >= 0 && y < self.map.height && (y + height) < self.map.height{
            for j in y..(y + height){
                self.map.change_tile(x, j, Tile::new(false, '.', colors::GREY));
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

                if wall_chance < 45{
                    self.map.change_tile(i, j, Tile::new(true, '#', colors::WHITE));
                }
                else{
                    self.map.change_tile(i, j, Tile::new(false, '.', colors::WHITE));
                }
            }
        }

        for iteration in 0..5{
            for j in 0..self.map.height{
                for i in 0..self.map.width{
                    let neighbours = self.map.get_neighbours(i, j);
                    if neighbours.len() < 8{
                        self.map.change_tile(i, j, Tile::new(true, '#', colors::WHITE));
                    }
                    else{
                        if MapBuilder::count_walls(&neighbours) < 5{
                            self.map.change_tile(i, j, Tile::new(true, '#', colors::WHITE));
                        }
                        else{
                            self.map.change_tile(i, j, Tile::new(false, '.', colors::WHITE));
                        }
                    }
                }
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
