use crate::{Configuration, mutation::Mutation};

use std::fmt::{Display, Formatter, Error};

use rand::Rng;

#[derive(Default, Debug, Clone)]
pub struct Swap {
    iterations : usize,
    chunk_size : usize,
    ranges : Ranges,
}

#[derive(Default, Debug, Clone)]
struct Ranges {
    it_range : (usize, usize),
    ch_range : (usize, usize),
}

impl Display for Swap {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "SWP_it={}_ch={}", self.iterations, self.chunk_size)
    }
}

impl Mutation for Swap {
    crate::impl_configure!(
        "SwapConfig",
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

        let sl = data
                .get_mut(index_min..index_max)
                .unwrap();

        let len = sl.len();

        for _ in 0..self.iterations {
            let splitdex = rng.gen_range(self.chunk_size, index_max-index_min-self.chunk_size);

            let (left, right) = sl.split_at_mut(splitdex);

            let index1 = rng.gen_range(0, splitdex - self.chunk_size);
            let index2 = rng.gen_range(0, len - splitdex - self.chunk_size);
            let (llen, rlen) = {(left.len(), right.len())};

            let slice1 = left.get_mut(index1..index1+self.chunk_size);
            let slice2 = right.get_mut(index2..index2+self.chunk_size);

            if slice1.is_none() || slice2.is_none() {
                eprintln!("Diagnostics before panic.");
                eprintln!("i1r:{}, i2r:{}", splitdex - self.chunk_size, len - splitdex - self.chunk_size);
                eprintln!("flen:{}, dex:{}, ch:{}", len, splitdex, self.chunk_size);
                eprintln!("len:{}, i1a/i1b:{}/{}", llen, index1, index1+self.chunk_size);
                eprintln!("len:{}, i2a/i2b:{}/{}", rlen, index2, index2+self.chunk_size);
                panic!("Out of bounds error. If you see this, please contact the developer.");
            }

            slice1.unwrap().swap_with_slice(slice2.unwrap());
        }
    }
}
