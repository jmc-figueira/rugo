pub struct Stats{
    pub max_hp: u64,
    pub curr_hp: u64,
    pub strength: u64,
    pub dexterity: u64,
    pub constitution: u64,
    pub intelligence: u64,
}

impl Stats{
    pub fn new() -> Stats{
        Stats{
            max_hp: 15,
            curr_hp: 15,
            strength: 10,
            dexterity: 12,
            constitution: 14,
            intelligence: 19
        }
    }
}

impl Clone for Stats{
    fn clone(&self) -> Stats{
        Stats{
            max_hp: self.max_hp,
            curr_hp: self.curr_hp,
            strength: self.strength,
            dexterity: self.dexterity,
            constitution: self.constitution,
            intelligence: self.intelligence
        }
    }
}

pub trait StatDriven{
    fn get_stats(&self) -> &Stats;
}
