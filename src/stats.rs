pub trait Stats{
    max_hp: u64,
    curr_hp: u64,
}

impl Stats{
    fn new() -> Stats{
        Stats{
            max_hp: 15,
            curr_hp: 15,
        }
    }
}
