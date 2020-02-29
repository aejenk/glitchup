use crate::{Configuration, mutation::Mutation};

use std::fmt::{Display, Formatter, Error};

use rand::Rng;

#[derive(Default, Debug, Clone)]
pub struct Compress {
    iterations : usize,
    chunk_size : usize,
    compress_by : usize,
    ranges : Ranges,
}

#[derive(Default, Debug, Clone)]
struct Ranges {
    it_range : (usize, usize),
    ch_range : (usize, usize),
    cp_range : (usize, usize),
}

impl Display for Compress {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "CPS_it={}_ch={}_cp={}", self.iterations, self.chunk_size, self.compress_by)
    }
}

impl Mutation for Compress {
    crate::impl_configure!(
        "CompressConfig",
        ["iterations", "chunksize", "compressby"],
        [it_range, ch_range, cp_range]
    );

    fn mutate(&mut self, data: &mut [u8]) {
        // random number generator
        let mut rng = rand::thread_rng();
        let (index_min, index_max) = super::index_boundary(data);

        crate::rangeinit!(self, rng,
            [it_range => iterations,
             ch_range => chunk_size,
             cp_range => compress_by]);

        for _ in 0..self.iterations {
            let index = rng.gen_range(index_min, index_max);

            let endindex = if self.chunk_size + index > data.len() {data.len()} else {self.chunk_size + index};

            if let Some(slice) = data.get_mut(index..endindex) {
                let mut c_index = 0; // index of byte to use

                let mut sclone = vec![0; slice.len()];
                sclone[..].clone_from_slice(slice);
                let slen = sclone.len();

                for chr in slice.iter_mut() {
                    *chr = sclone[c_index];
                    c_index += self.compress_by;

                    if c_index >= slen {
                        break;
                    }
                }
            }
        }
    }
}
