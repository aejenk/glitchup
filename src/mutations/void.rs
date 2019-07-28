use glitchconsole::{
    mutation::Mutation,
    options::MutConfig
};

use std::fmt::{Display, Formatter, Error};

use rand::Rng;

#[derive(Default)]
pub struct Void {
    iterations : u64,
    chunk_size : usize,
    ranges : Ranges,
}

#[derive(Default)]
struct Ranges {
    it_range : (u64, u64),
    ch_range : (usize, usize),
}

impl Display for Void {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "VOID_iter={}_csize={}", self.iterations, self.chunk_size)
    }
}

impl Mutation for Void {
    fn configure(&mut self, config: Box<&dyn MutConfig>) {
        use glitchconsole::options::MutOptionVal::*;

        let mutopts = &config.to_hashmap();

        // Sets the Iterations range
        if let OArray(range) = &mutopts["iterations"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.it_range = (*min as u64, *max as u64);
            }
            else {
                panic!("Iterations should be a list of numbers.");
            }
        } else {
            panic!("Iterations (Vec) is a required option. Please set it globally.");
        }

        // Sets the Chunksize range
        if let OArray(range) = &mutopts["chunksize"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.ch_range = (*min as usize, *max as usize);
            }
            else {
                panic!("Chunksize should be a list of numbers.");
            }
        } else {
            panic!("Chunksize (Vec) is a required option. Please set it globally.");
        }
    }

    fn mutate(&mut self, data: &mut [u8]) {
        // random number generator
        let mut rng = rand::thread_rng();

        let (it_min, it_max) = self.ranges.it_range;
        let (ch_min, ch_max) = self.ranges.ch_range;

        let len = data.len();
        let (index_min, index_max) = (len/50, len);

        self.iterations = rng.gen_range(it_min, it_max);
        self.chunk_size = rng.gen_range(ch_min, ch_max);

        for _ in 0..self.iterations {
            let index = rng.gen_range(index_min, index_max);

            if let Some(slice) = data.get_mut(index..self.chunk_size+index) {
                for chr in slice.iter_mut() {
                    *chr = b'0';
                }
            }
        }
    }
}
