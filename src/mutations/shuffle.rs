use crate::{Configuration, mutation::Mutation};

use std::fmt::{Display, Formatter, Error};

use rand::Rng;
use rand::seq::SliceRandom;

#[derive(Default, Debug, Clone)]
pub struct Shuffle {
    iterations : usize,
    chunk_size : usize,
    ranges : Ranges,
}

#[derive(Default, Debug, Clone)]
struct Ranges {
    it_range : (usize, usize),
    ch_range : (usize, usize),
}

impl Display for Shuffle {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "SHF_it={}_ch={}", self.iterations, self.chunk_size)
    }
}

impl Mutation for Shuffle {
    crate::impl_configure!(
        "ShuffleConfig",
        ["iterations", "chunksize"],
        [it_range, ch_range]
    );

    fn mutate(&mut self, data: &mut [u8]) {
        // random number generator
        let mut rng = rand::thread_rng();
        let (index_min, index_max) = super::index_boundary(data);

        crate::rangeinit!(self, rng,
             [it_range => iterations,
              ch_range => chunk_size]
        );

        for _ in 0..self.iterations {
            let index = rng.gen_range(index_min, index_max);

            let endindex = if self.chunk_size + index > data.len() {data.len()} else {self.chunk_size + index};

            if let Some(slice) = data.get_mut(index..endindex){
                slice.shuffle(&mut rng);
            }
        }
    }
}
