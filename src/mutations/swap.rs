use glitchconsole::{
    mutation::Mutation,
    options::MutConfig
};

use std::fmt::{Display, Formatter, Error};

use rand::Rng;

#[derive(Default)]
pub struct Swap {
    iterations : u64,
    chunk_size : usize,
    ranges : Ranges,
}

#[derive(Default)]
struct Ranges {
    it_range : (u64, u64),
    ch_range : (usize, usize),
}

impl Display for Swap {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "SWP_it={}_ch={}", self.iterations, self.chunk_size)
    }
}

impl Mutation for Swap {
    fn configure(&mut self, config: Box<&dyn MutConfig>) {
        use glitchconsole::options::MutOptionVal::*;

        let mutopts = &config.to_hashmap();

        // Sets the Iterations range
        if let OArray(range) = &mutopts["iterations"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.it_range = (*min as u64, *max as u64);
            }
            else {
                panic!("\'iterations\' should be a list of numbers.");
            }
        } else {
            panic!("\'iterations\' (Vec) is a required option. Please set it globally.");
        }

        // Sets the Chunksize range
        if let OArray(range) = &mutopts["chunksize"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.ch_range = (*min as usize, *max as usize);
            }
            else {
                panic!("\'chunksize\' should be a list of numbers.");
            }
        } else {
            panic!("\'chunksize\' (Vec) is a required option. Please set it globally.");
        }
    }

    fn mutate(&mut self, data: &mut [u8]) {
        // random number generator
        let mut rng = rand::thread_rng();

        let (it_min, it_max) = self.ranges.it_range;
        let (ch_min, ch_max) = self.ranges.ch_range;
        let (index_min, index_max) = (data.len()/50, data.len());

        self.iterations = rng.gen_range(it_min, it_max);
        self.chunk_size = rng.gen_range(ch_min, ch_max);

        let sl = data
                .get_mut(index_min..index_max)
                .unwrap();

        let len = sl.len();

        for _ in 0..self.iterations {
            let splitdex = rng.gen_range(0, index_max-index_min);

            let (left, right) = sl.split_at_mut(splitdex);

            let index1 = rng.gen_range(0, splitdex - self.chunk_size);
            let index2 = rng.gen_range(0, len - splitdex - self.chunk_size);

            let (llen, rlen) = {(left.len(), right.len())};

            println!("len:{}, i1a/i1b:{}/{}", llen, index1, index1+self.chunk_size);
            println!("len:{}, i2a/i2b:{}/{}", rlen, index2, index2+self.chunk_size);

            let slice1 = left.get_mut(index1..index1+self.chunk_size).unwrap();
            let slice2 = right.get_mut(index2..index2+self.chunk_size).unwrap();

            slice1.swap_with_slice(slice2);
        }
    }
}
