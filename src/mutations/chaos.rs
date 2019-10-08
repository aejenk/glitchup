use glitchconsole::{
    mutation::Mutation,
    options::MutConfig
};

use std::fmt::{Display, Formatter, Error};

use rand::Rng;
use rand_xorshift::XorShiftRng;
use rand_core::SeedableRng;
use rand_core::RngCore;

#[derive(Default, Debug, Clone)]
pub struct Chaos {
    iterations : usize,
    chunk_size : usize,
    ranges : Ranges,
}

#[derive(Default, Debug, Clone)]
struct Ranges {
    it_range : (usize, usize),
    ch_range : (usize, usize),
}

impl Display for Chaos {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "CHS_it={}_ch={}", self.iterations, self.chunk_size)
    }
}

impl Mutation for Chaos {
    fn configure(&mut self, config: Box<&dyn MutConfig>) {
        use glitchconsole::options::MutOptionVal::*;

        let cfg = &config.to_hashmap();
        let chaoscfg = if let OMap(map) = &cfg["ChaosConfig"] {map} else {
            println!("not configuring CHAOS - not included.");
            return;
        };

        // Sets the Iterations range
        if let OArray(range) = &chaoscfg["iterations"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.it_range = (*min as usize, *max as usize);
            }
            else {panic!("ITERS not [INT,INT]")}
        } else {panic!("ITERS not ARR")};

        // Sets the Chunksize range
        if let OArray(range) = &chaoscfg["chunksize"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.ch_range = (*min as usize, *max as usize);
            }
            else {panic!("CHUNKSIZE not [INT,INT]")}
        } else {panic!("CHUNKSIZE not ARR")};
    }

    fn mutate(&mut self, data: &mut [u8]) {
        // random number generator
        let mut rng = rand::thread_rng();

        let (it_min, it_max) = self.ranges.it_range;
        let (ch_min, ch_max) = self.ranges.ch_range;

        let len = data.len();
        let (index_min, index_max) = (len/50, len);

        self.iterations = rng.gen_range(it_min, it_max);
        self.chunk_size = rng.gen_range(ch_min, ch_max);

        // Random number generator focused on speed.
        let mut xrng = XorShiftRng::from_rng(rng.clone()).unwrap();

        for _ in 0..self.iterations {
            let index = rng.gen_range(index_min, index_max);

            let endindex = if self.chunk_size + index > data.len() {data.len()} else {self.chunk_size + index};

            if let Some(slice) = data.get_mut(index..endindex) {
                for chr in slice.iter_mut() {
                    *chr = xrng.next_u32() as u8
                }
            }
        }
    }
}
