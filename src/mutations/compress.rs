use glitchconsole::{
    mutation::Mutation,
    options::MutConfig
};

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
    fn configure(&mut self, config: Box<&dyn MutConfig>) {
        use glitchconsole::options::MutOptionVal::*;

        let cfg = &config.to_hashmap();
        let compresscfg = if let OMap(map) = &cfg["CompressConfig"] {map} else {
            // println!("not configuring COMPRESS - not included.");
            return;
        };

        // Sets the Iterations range
        if let OArray(range) = &compresscfg["iterations"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.it_range = (*min as usize, *max as usize);
            }
            else {panic!("ITERS not [INT,INT]")}
        } else {panic!("ITERS not ARR")};

        // Sets the Chunksize range
        if let OArray(range) = &compresscfg["chunksize"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.ch_range = (*min as usize, *max as usize);
            }
            else {panic!("CHUNKSIZE not [INT,INT]")}
        } else {panic!("CHUNKSIZE not ARR")};

        
        // Sets the Compressby range
        if let OArray(range) = &compresscfg["compress_by"] {
            if let (OInt(min), OInt(max)) = (&range[0], &range[1]) {
                self.ranges.cp_range = (*min as usize, *max as usize);
            }
            else {panic!("COMPRESSBY not [INT,INT]")}
        } else {panic!("COMPRESSBY not ARR")};
    }

    fn mutate(&mut self, data: &mut [u8]) {
        // random number generator
        let mut rng = rand::thread_rng();

        let (it_min, it_max) = self.ranges.it_range;
        let (ch_min, ch_max) = self.ranges.ch_range;
        let (cp_min, cp_max) = self.ranges.cp_range;

        let len = data.len();
        let (index_min, index_max) = (len/50, len);

        self.iterations = rng.gen_range(it_min, it_max);
        self.chunk_size = rng.gen_range(ch_min, ch_max);
        self.compress_by = rng.gen_range(cp_min, cp_max);

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
