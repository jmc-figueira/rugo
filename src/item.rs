use object::*;
use colors::*;
use tcod::console::*;
use std::collections::HashMap;

pub struct Weapon{
    id: u64,
    name: String,
    pub x: i32,
    pub y: i32,
    graphic: char,
    color: ColorCell,
}

impl Weapon{
    pub fn new(id_gen: &mut IDManager, name: &str, x: i32, y: i32, graphic: char, bg: Rgb, fg: Rgb) -> Weapon{
        Weapon {
            id: id_gen.next_id(),
            name: String::from(name),
            x: x,
            y: y,
            graphic: graphic,
            color: ColorCell::new(bg, fg)
        }
    }

    pub fn get_id(&self) -> u64{
        self.id
    }

    pub fn get_name(&self) -> &str{
        self.name.as_str()
    }
}

impl Object for Weapon{
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

pub trait ItemList{
    fn add(&mut self, item: Weapon) -> u64;

    fn items_at(&self, x: i32, y: i32) -> Vec<&Weapon>;

    fn remove(&mut self, id: u64) -> Option<Weapon>;
}

pub struct ItemManager{
    list: HashMap<u64, Weapon>,
}

impl ItemManager{
    pub fn new() -> ItemManager{
        ItemManager{
            list: HashMap::new()
        }
    }

    pub fn render(&self, console: &mut Console){
        for item in self.list.values(){
            console.put_char_ex(item.x, item.y, item.graphic, *item.color.foreground(), *item.color.background());
        }
    }
}

impl ItemList for ItemManager{
    fn add(&mut self, item: Weapon) -> u64{
        let ret_val = item.get_id();
        self.list.insert(item.get_id(), item);
        ret_val
    }

    fn items_at(&self, x: i32, y: i32) -> Vec<&Weapon>{
        let mut ret_val = Vec::new();

        for item in self.list.values(){
            if item.get_coords().0 == x && item.get_coords().1 == y{
                ret_val.push(item);
            }
        }
        ret_val
    }

    fn remove(&mut self, id: u64) -> Option<Weapon>{
        self.list.remove(&id)
    }
}