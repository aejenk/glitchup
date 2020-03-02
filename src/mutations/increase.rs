use crate::{Configuration, mutation::Mutation};

use std::fmt::{Display, Formatter, Error};

use rand::Rng;

#[derive(Default, Debug, Clone)]
pub struct Increase {
    iterations : usize,
    chunk_size : usize,
    increase_by: usize,
    ranges : Ranges,
}

#[derive(Default, Debug, Clone)]
struct Ranges {
    it_range : (usize, usize),
    ch_range : (usize, usize),
    ic_range : (usize, usize)
}

impl Display for Increase {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "INC_it={}_ch={}_by={}", self.iterations, self.chunk_size, self.increase_by)
    }
}

impl Mutation for Increase {
    crate::impl_configure!(
        "IncreaseConfig",
        ["iterations", "chunksize", "increaseby"],
        [it_range, ch_range, ic_range]
    );

    fn mutate(&mut self, data: &mut [u8]) {
        // random number generator
        let mut rng = rand::thread_rng();
        let (index_min, index_max) = super::index_boundary(data);

        crate::rangeinit!(self, rng,
             [it_range => iterations,
              ch_range => chunk_size,
              ic_range => increase_by]
        );

        for _ in 0..self.iterations {
            let index = rng.gen_range(index_min, index_max);

            let endindex = if self.chunk_size + index > data.len() {data.len()} else {self.chunk_size + index};

            if let Some(slice) = data.get_mut(index..endindex) {
                for chr in slice.iter_mut() {
                    *chr = ((*chr as usize + self.increase_by) % 256) as u8;
                }
            }
        }
    }
}
