use rand;
use rand::Rng;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

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
        let mut ret_val = 0i64;
        for _ in 0..self.number_of_die{
            ret_val += rand::thread_rng().gen_range(1, self.number_of_faces as i64);
        }
        ret_val
    }
}

impl Display for SimpleDice{
    fn fmt(&self, f: &mut Formatter) -> Result{
        write!(f, "{}d{}", self.number_of_die, self.number_of_faces)
    }
}