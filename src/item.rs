use object::*;
use colors::*;
use tcod::console::*;
use std::collections::HashMap;

pub struct Item{
    id: u64,
    name: String,
    pub x: i32,
    pub y: i32,
    graphic: char,
    color: ColorCell,
}

impl Item{
    pub fn new(id_gen: &mut IDManager, name: &str, x: i32, y: i32, graphic: char, bg: Rgb, fg: Rgb) -> Item{
        Item{
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
}

impl Object for Item{
    fn render(&self, console: &mut Console){
        console.put_char_ex(self.x, self.y, self.graphic, *self.color.foreground(), *self.color.background());
    }
}

pub trait ItemList{
    fn add(&mut self, item: Item) -> u64;

    fn remove(&mut self, id: u64) -> Option<Item>;
}

pub struct ItemManager{
    list: HashMap<u64, Item>,
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
    fn add(&mut self, item: Item) -> u64{
        let ret_val = item.get_id();
        self.list.insert(item.get_id(), item);
        ret_val
    }

    fn remove(&mut self, id: u64) -> Option<Item>{
        self.list.remove(&id)
    }
}