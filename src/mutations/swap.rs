use glitchconsole::{
    mutation::Mutation,
    options::MutConfig
};

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
    fn configure(&mut self, config: Box<&dyn MutConfig>) {
        use glitchconsole::options::MutOptionVal::*;

        let cfg = &config.to_hashmap();
        let swapcfg = if let OMap(map) = &cfg["SwapConfig"] {map} else {
            // println!("not configuring SWAP - not included.");
            return;
        };

        // Sets the Iterations range
        if let OArray(range) = &swapcfg["iterations"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.it_range = (*min as usize, *max as usize);
            }
            else {panic!("ITERS not [INT,INT]")}
        } else {panic!("ITERS not ARR")};

        // Sets the Chunksize range
        if let OArray(range) = &swapcfg["chunksize"] {
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
        let (index_min, index_max) = (data.len()/50, data.len());

        self.iterations = rng.gen_range(it_min, it_max);
        self.chunk_size = rng.gen_range(ch_min, ch_max);

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
