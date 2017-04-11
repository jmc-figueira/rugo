use object::*;
use colors::*;
use dice::*;
use tcod::console::*;
use std::collections::HashMap;

pub trait Item : Object{
    fn get_name(&self) -> &str;

    fn get_id(&self) -> u64;
}

pub struct Weapon{
    id: u64,
    name: String,
    description: String,
    pub x: i32,
    pub y: i32,
    graphic: char,
    color: ColorCell,
    hit: Box<Dice>,
    damage: Box<Dice>,
    weight: i64,
}

impl Weapon{
    pub fn new(id_gen: &mut IDManager, name: &str, description: &str, x: i32, y: i32, graphic: char, bg: Rgb, fg: Rgb, hit_dice: Box<Dice>, damage_dice: Box<Dice>, weight: i64) -> Weapon{
        Weapon{
            id: id_gen.next_id(),
            name: String::from(name),
            description: String::from(description),
            x: x,
            y: y,
            graphic: graphic,
            color: ColorCell::new(bg, fg),
            hit: hit_dice,
            damage: damage_dice,
            weight: weight
        }
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

impl Item for Weapon{
    fn get_name(&self) -> &str{
        self.name.as_str()
    }

    fn get_id(&self) -> u64{
        self.id
    }
}

pub trait ItemList{
    fn add(&mut self, item: Box<Item>) -> u64;

    fn items_at(&self, x: i32, y: i32) -> Vec<&Box<Item>>;

    fn remove(&mut self, id: u64) -> Option<Box<Item>>;
}

pub struct ItemManager{
    list: HashMap<u64, Box<Item>>,
}

impl ItemManager{
    pub fn new() -> ItemManager{
        ItemManager{
            list: HashMap::new()
        }
    }

    pub fn render(&self, console: &mut Console){
        for item in self.list.values(){
            console.put_char_ex(item.get_coords().0, item.get_coords().1, item.get_graphic(), *item.get_color().foreground(), *item.get_color().background());
        }
    }
}

impl ItemList for ItemManager{
    fn add(&mut self, item: Box<Item>) -> u64{
        let ret_val = item.get_id();
        self.list.insert(item.get_id(), item);
        ret_val
    }

    fn items_at(&self, x: i32, y: i32) -> Vec<&Box<Item>>{
        let mut ret_val = Vec::new();

        for item in self.list.values(){
            if item.get_coords().0 == x && item.get_coords().1 == y{
                ret_val.push(item);
            }
        }
        ret_val
    }

    fn remove(&mut self, id: u64) -> Option<Box<Item>>{
        self.list.remove(&id)
    }
}