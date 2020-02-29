use crate::{Configuration, mutation::Mutation};

use std::fmt::{Display, Formatter, Error};

use rand::Rng;

#[derive(Default, Debug, Clone)]
pub struct Multiply {
    iterations : usize,
    chunk_size : usize,
    multiply_by: f64,
    ranges : Ranges,
}

#[derive(Default, Debug, Clone)]
struct Ranges {
    it_range : (usize, usize),
    ch_range : (usize, usize),
    ml_range : (f64, f64)
}

impl Display for Multiply {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "MAG_it={}_ch={}_by={}", self.iterations, self.chunk_size, self.multiply_by)
    }
}

impl Mutation for Multiply {
    crate::impl_configure!(
        "MultiplyConfig",
        ["iterations", "chunksize" | "multiplyby"],
        [it_range, ch_range | ml_range]
    );

    fn mutate(&mut self, data: &mut [u8]) {
        // random number generator
        let mut rng = rand::thread_rng();
        let (index_min, index_max) = super::index_boundary(data);

        crate::rangeinit!(self, rng,
             [it_range => iterations,
              ch_range => chunk_size,
              ml_range => multiply_by]
        );

        for _ in 0..self.iterations {
            let index = rng.gen_range(index_min, index_max);

            let endindex = if self.chunk_size + index > data.len() {data.len()} else {self.chunk_size + index};

            if let Some(slice) = data.get_mut(index..endindex) {
                for chr in slice.iter_mut() {
                    *chr = ((*chr as f64 * self.multiply_by) as usize % 256) as u8;
                }
            }
        }
    }
}
