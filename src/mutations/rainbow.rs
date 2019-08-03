use glitchconsole::{
    mutation::Mutation,
    options::MutConfig
};

use std::fmt::{Display, Formatter, Error};

use rand::Rng;

#[derive(Default)]
pub struct Rainbow {
    iterations : u64,
    chunk_size : usize,
    accelerate_by : usize,
    accelerate_in : usize,
    ranges : Ranges,
}

#[derive(Default)]
struct Ranges {
    it_range : (u64, u64),
    ch_range : (usize, usize),
    ab_range : (usize, usize),
    ai_range : (usize, usize),
}

impl Display for Rainbow {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "RBW_it={}_ch={}_by={}_in={}", self.iterations, self.chunk_size, self.accelerate_by, self.accelerate_in)
    }
}

impl Mutation for Rainbow {
    fn configure(&mut self, config: Box<&dyn MutConfig>) {
        use glitchconsole::options::MutOptionVal::*;

        let mutopts = &config.to_hashmap();

        let raiopts = if let Some(OMap(map)) = &mutopts.get("rainbow_mut") {
            map
        } else {
            panic!("Sub-options for 'Rainbow' not found. Please add them under '[rainbow_mut]'")
        };

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

        // Sets the AccBy range
        if let OArray(range) = &raiopts["accelerate_by"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.ab_range = (*min as usize, *max as usize);
            }
            else {
                panic!("\'accelerate_by\' should be a list of numbers.");
            }
        } else {
            panic!("\'accelerate_by\' (Vec) is a required option. Please set it under '[rainbow_mut]'.");
        }

        // Sets the Iterations range
        if let OArray(range) = &raiopts["accelerate_in"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.ai_range = (*min as usize, *max as usize);
            }
            else {
                panic!("\'accelerate_in\' should be a list of numbers.");
            }
        } else {
            panic!("\'accelerate_in\' (Vec) is a required option. Please set it under '[rainbow_mut]'.");
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
        let (ab_min, ab_max) = self.ranges.ab_range;
        let (ai_min, ai_max) = self.ranges.ai_range;

        let len = data.len();
        let (index_min, index_max) = (len/50, len);

        self.iterations = rng.gen_range(it_min, it_max);
        self.chunk_size = rng.gen_range(ch_min, ch_max);
        self.accelerate_by = rng.gen_range(ab_min, ab_max);
        self.accelerate_in = rng.gen_range(ai_min, ai_max);

        for _ in 0..self.iterations {
            let index = rng.gen_range(index_min, index_max);

            let mut n = self.accelerate_by;
            let mut i = 0;

            if let Some(slice) = data.get_mut(index..self.chunk_size+index) {
                for chr in slice.iter_mut() {
                    *chr = ((*chr as usize + n) % 256) as u8;
                    i += 1;

                    if i >= self.accelerate_in {
                        i = 0;
                        n += self.accelerate_by;
                    };
                }
            }
        }
    }
}
