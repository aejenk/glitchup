use glitchconsole::{
    mutation::Mutation,
    options::MutConfig
};

use std::fmt::{Display, Formatter, Error};

use rand::Rng;

#[derive(Default, Debug, Clone)]
pub struct Loops {
    iterations : usize,
    chunk_size : usize,
    loops : usize,
    ranges : Ranges,
}

#[derive(Default, Debug, Clone)]
struct Ranges {
    it_range : (usize, usize),
    lp_range : (usize, usize),
    ch_range : (usize, usize),
}

impl Display for Loops {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "LOOP_it={}_ch={}_lps={}", self.iterations, self.chunk_size, self.loops)
    }
}

impl Mutation for Loops {
    fn configure(&mut self, config: Box<&dyn MutConfig>) {
        use glitchconsole::options::MutOptionVal::*;

        let cfg = &config.to_hashmap();
        let loopcfg = if let OMap(map) = &cfg["LoopConfig"] {map} else {
            // println!("not configuring LOOPS - not included.");
            return;
        };

        // Sets the Iterations range
        if let OArray(range) = &loopcfg["iterations"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.it_range = (*min as usize, *max as usize);
            }
            else {panic!("ITERS not [INT,INT]")}
        } else {panic!("ITERS not ARR")};

        // Sets the Chunksize range
        if let OArray(range) = &loopcfg["chunksize"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.ch_range = (*min as usize, *max as usize);
            }
            else {panic!("CHUNKSIZE not [INT,INT]")}
        } else {panic!("CHUNKSIZE not ARR")};

        // Sets the Loops range
        if let OArray(range) = &loopcfg["loops"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.lp_range = (*min as usize, *max as usize);
            }
            else {panic!("LOOPS not [INT,INT]")}
        } else {panic!("LOOPS not ARR")};
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
            if let Some(slice) = data.get_mut(0..len) {
                // Loop for (self.chunk_size) times...
                for _ in 0..self.chunk_size {
                    // Internally loop (self.loop) times...
                    for rep in 1..=self.loops {
                        // Get the index of the character to modify
                        let modind = 
                            if index + self.chunk_size * rep < index_max{
                                index + self.chunk_size * rep
                            }
                            else {
                                ((index + self.chunk_size * rep) % (index_max-index_min)) + index_min
                            };

                        // Shows important info before panic - for catching bugs.
                        if index > len || modind > len {
                            eprintln!("Diagnostics before panic.");
                            eprintln!("index:{}, min/max:{}/{}, modind:{}, chsize:{}, in+ch+tp:{} % max {} + min {}",
                             index, index_min, index_max, modind, self.chunk_size, index + self.chunk_size * rep,
                             (index + self.chunk_size * rep) % index_max, ((index + self.chunk_size * rep) % index_max) + index_min);
                            panic!("Out of bounds error. If you see this, please contact the developer.");
                        }
                            
                        // "Repeat" current byte across other byte.
                        slice[modind] = slice[index];
                    }
                }
            }
        }
    }
}
