use glitchconsole::{
    mutation::Mutation,
    options::MutConfig
};

use std::fmt::{Display, Formatter, Error};

use rand::Rng;

#[derive(Default)]
pub struct Loops {
    iterations : usize,
    chunk_size : usize,
    loops : usize,
    ranges : Ranges,
}

#[derive(Default)]
struct Ranges {
    it_range : (usize, usize),
    lp_range : (usize, usize),
    ch_range : (usize, usize),
}

impl Display for Loops {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "LOOP_iter={}_csize={}_loops={}", self.iterations, self.chunk_size, self.loops)
    }
}

impl Mutation for Loops {
    fn configure(&mut self, config: Box<&dyn MutConfig>) {
        use glitchconsole::options::MutOptionVal::*;

        

        let mutopts = &config.to_hashmap();
        let loopopts = if let Some(OMap(map)) = &mutopts.get("loop_mut") {
            map
        } else {
            panic!("Sub-options for \"Loop\" were not found.");
        };

        // Sets the Iterations range
        if let OArray(range) = &mutopts["iterations"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.it_range = (*min as usize, *max as usize);
            }
            else {
                panic!("\'iterations\' should be a list of numbers.");
            }
        } else {
            panic!("\'iterations\' (Vec) is a required option. Please set it globally.");
        }

        // Sets the Loops range
        if let OArray(range) = &loopopts["loops"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.lp_range = (*min as usize, *max as usize);
            }
            else {
                panic!("\'loops\' should be a list of numbers.");
            }
        } else {
            panic!("\'loops\' (Vec) is a required option. Please set it under [loops].");
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
        let (lp_min, lp_max) = self.ranges.lp_range;

        let len = data.len();
        let (index_min, index_max) = (len/50, len);

        self.iterations = rng.gen_range(it_min, it_max);
        self.chunk_size = rng.gen_range(ch_min, ch_max);
        self.loops      = rng.gen_range(lp_min, lp_max);

        for _ in 0..self.iterations {
            let index = rng.gen_range(index_min, index_max);

            // Get whole file to allow circular access
            if let Some(slice) = data.get_mut(index_min..index_max) {
                // Loop for (self.chunk_size) times...
                for _ in 0..self.chunk_size {
                    // Internally loop (self.loop) times...
                    for rep in 1..=self.loops {
                        // Get the index of the character to modify
                        let modind = 
                            if index + self.chunk_size * rep < index_max 
                                {index + self.chunk_size * rep}
                            else
                                {((index + self.chunk_size * rep) % index_max) + index_min};
                            
                        // "Repeat" current byte across other byte.
                        slice[modind] = slice[index];
                    }
                }
            }
        }
    }
}
