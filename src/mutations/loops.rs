use crate::{Configuration, mutation::Mutation};


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
    crate::impl_configure!(
        "LoopsConfig",
        ["iterations", "chunksize", "loops"],
        [it_range, ch_range, lp_range]
    );

    fn mutate(&mut self, data: &mut [u8]) {
        // random number generator
        let mut rng = rand::thread_rng();
        let (index_min, index_max) = super::index_boundary(data);

        crate::rangeinit!(self, rng,
             [it_range => iterations,
              ch_range => chunk_size,
              lp_range => loops]
        );

        let len = data.len();

        for _ in 0..self.iterations {
            let index = rng.gen_range(index_min, index_max);

            // Get whole file to allow circular access
            if let Some(slice) = data.get_mut(0..) {
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
