use crate::{Configuration, mutation::Mutation};

use std::fmt::{Display, Formatter, Error};

use moveslice::Moveslice;

use rand::Rng;

#[derive(Default, Debug, Clone)]
pub struct Shift {
    iterations : usize,
    chunk_size : usize,
    ranges : Ranges,
}

#[derive(Default, Debug, Clone)]
struct Ranges {
    it_range : (usize, usize),
    ch_range : (usize, usize),
}

impl Display for Shift {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "SFT_it={}_ch={}", self.iterations, self.chunk_size)
    }
}

impl Mutation for Shift {
    crate::impl_configure!(
        "ShiftConfig",
        ["iterations", "chunksize"],
        [it_range, ch_range]
    );

    fn mutate(&mut self, data: &mut [u8]) {
        // random number generator
        let mut rng = rand::thread_rng();
        let (index_min, index_max) = super::index_boundary(data);
        let new_max = index_max - index_min;

        crate::rangeinit!(self, rng,
             [it_range => iterations,
              ch_range => chunk_size]
        );

        for _ in 0..self.iterations {
            let index = rng.gen_range(0, new_max);
            let m_index = rng.gen_range(0, new_max - self.chunk_size);

            if let Some(mut slice) = data.get_mut(index_min..index_max){
                let max_i = if self.chunk_size+index > slice.len() {slice.len()} else {self.chunk_size+index};
                let result = slice.try_moveslice(
                    index..max_i,
                    m_index
                );

                if let Err(res) = result {
                    eprintln!("Shifting failed. Moveslice returned error: {:?}", res);
                };
            }
        }
    }
}
