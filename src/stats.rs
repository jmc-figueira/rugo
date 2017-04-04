pub struct Stats{
    pub max_hp: u64,
    pub curr_hp: u64,
}

impl Stats{
    pub fn new() -> Stats{
        Stats{
            max_hp: 15,
            curr_hp: 15
        }
    }
}

impl Clone for Stats{
    fn clone(&self) -> Stats{
        Stats{
            max_hp: self.max_hp,
            curr_hp: self.curr_hp
        }
    }
}

pub trait StatDriven{
    fn get_stats(&self) -> &Stats;
}
