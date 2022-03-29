use rand::{Rng};
use crate::burmalda::{NUMBER_OF_LINES, NUMBER_OF_STEPS};

pub struct Bombs([usize; NUMBER_OF_STEPS]);

impl Bombs {

    pub fn new() -> Bombs {
        let mut rng = rand::thread_rng();
        let mut bombs = [0; NUMBER_OF_STEPS];
        for i in 0..NUMBER_OF_STEPS {
            bombs[i] = rng.gen_range(1..=NUMBER_OF_LINES);
        }
        Bombs(bombs)
    }

    pub fn get(&self, step: usize) -> usize {
        self.0[step]
    }
}