pub trait Dice{
    fn roll(&self) -> i64;
}

pub struct SimpleDice{
    number_of_die: u64,
    number_of_faces: u64,
}

impl SimpleDice{
    pub fn new(die: u64, faces: u64) -> SimpleDice{
        SimpleDice{
            number_of_die: die,
            number_of_faces: faces
        }
    }
}

impl Dice for SimpleDice{
    fn roll(&self) -> i64{
        
    }
}