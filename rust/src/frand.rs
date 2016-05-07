extern crate rand;
use std::clone::Clone;

const SIZE: usize = 16*1024;

pub struct FastRand {
    i: usize,
    buffer: [f64; SIZE],
}

impl Clone for FastRand {
  fn clone(&self) -> FastRand {
    FastRand { i: self.i, buffer: self.buffer}
  }
}

impl FastRand {
    pub fn new() -> FastRand {
        FastRand { i: 0, buffer: [0.0; SIZE]}
    }

    pub fn initialize(&mut self) {
        for i in 0..self.buffer.len() {
            self.buffer[i] = rand::random::<f64>();
        }
        self.i = 0;
    }

    pub fn seed(&mut self) {
        self.i = rand::random::<usize>();
        self.i = self.i % self.buffer.len();
    }

    pub fn get(&mut self) -> f64 {
        self.i = (self.i+1) % self.buffer.len();
        self.buffer[self.i]
    }
}
