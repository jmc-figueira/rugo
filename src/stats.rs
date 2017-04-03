pub struct Stats{
    pub max_hp: u64,
    pub curr_hp: u64,
}

impl Stats{
    pub fn new() -> Stats{
        Stats{
            max_hp: 15,
            curr_hp: 15,
        }
    }
}

pub trait StatDriven{
    fn set_hp(&mut self, hp: u64);

    fn set_max_hp(&mut self, max_hp: u64);

    fn get_hp(&self) -> u64;

    fn get_max_hp(&self) -> u64;
}
