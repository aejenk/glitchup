use crate::{Configuration, mutation::Mutation};

use std::fmt::{Display, Formatter, Error};

use rand::Rng;

#[derive(Default, Debug, Clone)]
pub struct Gradient {
    iterations : usize,
    chunk_size : usize,
    accelerate_by : usize,
    accelerate_in : usize,
    ranges : Ranges,
}

#[derive(Default, Debug, Clone)]
struct Ranges {
    it_range : (usize, usize),
    ch_range : (usize, usize),
    ab_range : (usize, usize),
    ai_range : (usize, usize),
}

impl Display for Gradient {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "GRT_it={}_ch={}_by={}_in={}", self.iterations, self.chunk_size, self.accelerate_by, self.accelerate_in)
    }
}

impl Mutation for Gradient {
    crate::impl_configure!(
        "GradientConfig",
        ["iterations", "chunksize", "accelerateby", "acceleratein"],
        [it_range, ch_range, ab_range, ai_range]
    );

    fn mutate(&mut self, data: &mut [u8]) {
        // random number generator
        let mut rng = rand::thread_rng();
        let (index_min, index_max) = super::index_boundary(data);

        crate::rangeinit!(self, rng,
             [it_range => iterations,
              ch_range => chunk_size,
              ai_range => accelerate_in,
              ab_range => accelerate_by]
        );

        for _ in 0..self.iterations {
            let index = rng.gen_range(index_min, index_max);

            let mut n = self.accelerate_by;
            let mut i = 0;

            let endindex = if self.chunk_size + index > data.len() {data.len()} else {self.chunk_size + index};

            if let Some(slice) = data.get_mut(index..endindex) {
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
