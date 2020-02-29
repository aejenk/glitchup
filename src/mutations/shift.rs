use glitchconsole::{
    mutation::Mutation,
    options::MutConfig
};

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
    fn configure(&mut self, config: Box<&dyn MutConfig>) {
        use glitchconsole::options::MutOptionVal::*;

        let cfg = &config.to_hashmap();
        let shiftcfg = if let OMap(map) = &cfg["ShiftConfig"] {map} else {
            // println!("not configuring SHIFT - not included.");
            return;
        };

        // Sets the Iterations range
        if let OArray(range) = &shiftcfg["iterations"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.it_range = (*min as usize, *max as usize);
            }
            else {panic!("ITERS not [INT,INT]")}
        } else {panic!("ITERS not ARR")};

        // Sets the Chunksize range
        if let OArray(range) = &shiftcfg["chunksize"] {
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
        let new_max = index_max - index_min;

        self.iterations = rng.gen_range(it_min, it_max);
        self.chunk_size = rng.gen_range(ch_min, ch_max);

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
