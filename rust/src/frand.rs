extern crate rand;
//rand::random::<f64>()

const SIZE: usize = 16*16*16*16;

pub struct FastRand {
    i: usize,
    buffer: [f64; SIZE],
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

    pub fn get(&mut self) -> f64 {
        self.i = (self.i+1) % self.buffer.len();
        self.buffer[self.i]
    }
}
