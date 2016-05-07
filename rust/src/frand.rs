extern crate rand;
use std::clone::Clone;

const SIZE: usize = 64*1024;

pub struct FastRand {
    i: usize,
    turns: usize,
    buffer: [f64; SIZE],
}

impl Clone for FastRand {
  fn clone(&self) -> FastRand {
    FastRand { i: self.i, turns: self.turns, buffer: self.buffer}
  }
}

impl FastRand {
    pub fn new() -> FastRand {
        let mut x = FastRand { i: 0, turns: 0, buffer: [0.0; SIZE]};
        x.initialize();
        x
    }

    pub fn initialize(&mut self) {
        for i in 0..SIZE {
            self.buffer[i] = rand::random::<f64>();
            //self.buffer[i] = (i as f64) / (SIZE as f64);
        }
        self.i = 0;
        self.turns = 0;
    }

#[inline]
    pub fn get(&mut self) -> f64 {
        self.i += 1;
        if self.i >= SIZE {
            self.i = 0;
            self.turns += 1;
        }
        self.buffer[self.i]
    }

#[inline]
    pub fn get_turns(&self) -> usize {
        self.turns
    }
}
