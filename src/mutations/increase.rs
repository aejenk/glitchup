use glitchconsole::{
    mutation::Mutation,
    options::MutConfig
};

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
    fn configure(&mut self, config: Box<&dyn MutConfig>) {
        use glitchconsole::options::MutOptionVal::*;

        let cfg = &config.to_hashmap();
        let increasecfg = if let OMap(map) = &cfg["IncreaseConfig"] {map} else {
            println!("not configuring INCREASE - not included.");
            return;
        };

        // Sets the Iterations range
        if let OArray(range) = &increasecfg["iterations"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.it_range = (*min as usize, *max as usize);
            }
            else {panic!("ITERS not [INT,INT]")}
        } else {panic!("ITERS not ARR")};

        // Sets the Chunksize range
        if let OArray(range) = &increasecfg["chunksize"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.ch_range = (*min as usize, *max as usize);
            }
            else {panic!("CHUNKSIZE not [INT,INT]")}
        } else {panic!("CHUNKSIZE not ARR")};

        // Sets the Chunksize range
        if let OArray(range) = &increasecfg["increase_by"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.ic_range = (*min as usize, *max as usize);
            }
            else {panic!("INCREASEBY not [INT,INT]")}
        } else {panic!("INCREASEBY not ARR")};
    }

    fn mutate(&mut self, data: &mut [u8]) {
        // random number generator
        let mut rng = rand::thread_rng();

        let (it_min, it_max) = self.ranges.it_range;
        let (ch_min, ch_max) = self.ranges.ch_range;
        let (in_min, in_max) = self.ranges.ic_range;

        let len = data.len();
        let (index_min, index_max) = (len/50, len);

        self.iterations = rng.gen_range(it_min, it_max);
        self.chunk_size = rng.gen_range(ch_min, ch_max);
        self.increase_by= rng.gen_range(in_min, in_max);

        for _ in 0..self.iterations {
            let index = rng.gen_range(index_min, index_max);

            if let Some(slice) = data.get_mut(index..self.chunk_size+index) {
                for chr in slice.iter_mut() {
                    *chr = ((*chr as usize + self.increase_by) % 256) as u8;
                }
            }
        }
    }
}
