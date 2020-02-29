use glitchconsole::{
    mutation::Mutation,
    options::MutConfig
};

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
    fn configure(&mut self, config: Box<&dyn MutConfig>) {
        use glitchconsole::options::MutOptionVal::*;

        let cfg = &config.to_hashmap();
        let multiplycfg = if let OMap(map) = &cfg["MultiplyConfig"] {map} else {
            // println!("not configuring INCREASE - not included.");
            return;
        };

        // Sets the Iterations range
        if let OArray(range) = &multiplycfg["iterations"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.it_range = (*min as usize, *max as usize);
            }
            else {panic!("ITERS not [INT,INT]")}
        } else {panic!("ITERS not ARR")};

        // Sets the Chunksize range
        if let OArray(range) = &multiplycfg["chunksize"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.ch_range = (*min as usize, *max as usize);
            }
            else {panic!("CHUNKSIZE not [INT,INT]")}
        } else {panic!("CHUNKSIZE not ARR")};

        // Sets the MultiplyBy range
        if let OArray(range) = &multiplycfg["multiply_by"] {
            if let (OFloat(min), OFloat(max)) = (&range[0], &range[1]) {
                self.ranges.ml_range = (*min as f64, *max as f64);
            }
            else {panic!("MULTIPLYBY not [FLOAT,FLOAT]")}
        } else {panic!("MULTIPLYBY not ARR")};
    }

    fn mutate(&mut self, data: &mut [u8]) {
        // random number generator
        let mut rng = rand::thread_rng();

        let (it_min, it_max) = self.ranges.it_range;
        let (ch_min, ch_max) = self.ranges.ch_range;
        let (ml_min, ml_max) = self.ranges.ml_range;

        let len = data.len();
        let (index_min, index_max) = (len/50, len);

        self.iterations = rng.gen_range(it_min, it_max);
        self.chunk_size = rng.gen_range(ch_min, ch_max);
        self.multiply_by= rng.gen_range(ml_min, ml_max);

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
