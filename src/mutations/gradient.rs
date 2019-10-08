use glitchconsole::{
    mutation::Mutation,
    options::MutConfig
};

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
    fn configure(&mut self, config: Box<&dyn MutConfig>) {
        use glitchconsole::options::MutOptionVal::*;

        let cfg = &config.to_hashmap();
        let gradientcfg = if let OMap(map) = &cfg["GradientConfig"] {map} else {
            println!("not configuring GRADIENT - not included.");
            return;
        };

        // Sets the Iterations range
        if let OArray(range) = &gradientcfg["iterations"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.it_range = (*min as usize, *max as usize);
            }
            else {panic!("ITERS not [INT,INT]")}
        } else {panic!("ITERS not ARR")};

        // Sets the Chunksize range
        if let OArray(range) = &gradientcfg["chunksize"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.ch_range = (*min as usize, *max as usize);
            }
            else {panic!("CHUNKSIZE not [INT,INT]")}
        } else {panic!("CHUNKSIZE not ARR")};

        // Sets the accelerate-by range
        if let OArray(range) = &gradientcfg["accelerate_by"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.ab_range = (*min as usize, *max as usize);
            }
            else {panic!("ACCELERATEBY not [INT,INT]")}
        } else {panic!("ACCELERATEBY not ARR")};

        // Sets the accelerate-in range
        if let OArray(range) = &gradientcfg["accelerate_in"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.ai_range = (*min as usize, *max as usize);
            }
            else {panic!("ACCELERATEIN not [INT,INT]")}
        } else {panic!("ACCELERATEIN not ARR")};
    }

    fn mutate(&mut self, data: &mut [u8]) {
        // random number generator
        let mut rng = rand::thread_rng();

        let (it_min, it_max) = self.ranges.it_range;
        let (ch_min, ch_max) = self.ranges.ch_range;
        let (ab_min, ab_max) = self.ranges.ab_range;
        let (ai_min, ai_max) = self.ranges.ai_range;

        let len = data.len();
        let (index_min, index_max) = (len/50, len);

        self.iterations = rng.gen_range(it_min, it_max);
        self.chunk_size = rng.gen_range(ch_min, ch_max);
        self.accelerate_by = rng.gen_range(ab_min, ab_max);
        self.accelerate_in = rng.gen_range(ai_min, ai_max);

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
