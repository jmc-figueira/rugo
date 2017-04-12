use object::*;
use colors::*;
use dice::*;
use tcod::console::*;
use std::collections::HashMap;
use std::collections::hash_map::Values;

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

    fn remove(&mut self, id: u64) -> Option<Box<Item>>;

    fn get_item_by_id(&self, id: u64) -> Option<&Box<Item>>;

    fn transfer(&mut self, id: u64, destination: &mut ItemList){
        destination.add(self.remove(id).unwrap());
    }
}

pub struct ItemManager{
    items: HashMap<u64, Box<Item>>,
}

impl ItemManager{
    pub fn new() -> ItemManager{
        ItemManager{
            items: HashMap::new()
        }
    }

    pub fn render(&self, console: &mut Console){
        for item in self.items.values(){
            console.put_char_ex(item.get_coords().0, item.get_coords().1, item.get_graphic(), *item.get_color().foreground(), *item.get_color().background());
        }
    }

    pub fn items_at(&self, x: i32, y: i32) -> Vec<u64>{
        let mut ret_val = Vec::new();

        for (id, item) in self.items.iter(){
            if item.get_coords().0 == x && item.get_coords().1 == y{
                ret_val.push(*id);
            }
        }
        ret_val
    }
}

impl ItemList for ItemManager{
    fn add(&mut self, item: Box<Item>) -> u64{
        let ret_val = item.get_id();
        self.items.insert(ret_val, item);
        ret_val
    }

    fn remove(&mut self, id: u64) -> Option<Box<Item>>{
        self.items.remove(&id)
    }

    fn get_item_by_id(&self, id: u64) -> Option<&Box<Item>> {
        self.items.get(&id)
    }
}

pub struct Inventory{
    items: HashMap<u64, Box<Item>>,
}

impl Inventory{
    pub fn new() -> Inventory{
        Inventory{
            items: HashMap::new()
        }
    }

    pub fn num_of_items(&self) -> usize{
        self.items.len()
    }

    pub fn items_iter(&self) -> Values<u64, Box<Item>>{
        self.items.values()
    }
}

impl ItemList for Inventory{
    fn add(&mut self, item: Box<Item>) -> u64 {
        let ret_val = item.get_id();
        self.items.insert(ret_val, item);
        ret_val
    }

    fn remove(&mut self, id: u64) -> Option<Box<Item>> {
        self.items.remove(&id)
    }

    fn get_item_by_id(&self, id: u64) -> Option<&Box<Item>>{
        self.items.get(&id)
    }
}