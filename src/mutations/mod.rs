use rand::Rng;
use super::options::MutConfig;

pub trait Mutation {
    fn mutate(&mut self, data : &mut [u8], config : Box<MutConfig>);
}

#[derive(Default)]
pub struct BasicMutation {
    min : usize,
    max : usize,
    chunk_size : usize
}

impl BasicMutation {
    fn process_options(&mut self, data: &[u8], config: Box<MutConfig>) {
        // to avoid verbosity
        use super::options::MutOptionVal::*;

        let mutopts = &config.to_hashmap();

        let options = match &mutopts["mutation"] {
            OMap(map) => map,
            _ => panic!("'mutation' should be an OMap(HashMap<String, MutOptionVal>)")
        };

        // matching values
        if let OInt(omin) = &options["min"] {
            self.min = *omin as usize;
        }
        else {
            self.min = 0;
        }

        if let OInt(csize) = &options["chunksize"] {
            self.chunk_size = *csize as usize;
        }
        else {
            panic!("BasicMutation: 'chunk_size [OInt]' not passed, is required!");
        }

        if let OInt(omax) = &options["max"] {
            if *omax as usize > data.len() - self.chunk_size {
                self.max = data.len() - self.chunk_size;
            }
            else {
                self.max = *omax as usize;
            }
        }
        else {
            self.max = data.len() - self.chunk_size;
        }
    }
}

impl Mutation for BasicMutation {
    fn mutate(&mut self, data: &mut [u8], config: Box<MutConfig>) {
        // random number generator
        let mut rng = rand::thread_rng();

        // processing configutation
        self.process_options(data, config);

        let index: usize = rng.gen_range(self.min, self.max);

        if let Some(slice) = data.get_mut(index..self.chunk_size+index) {
            for chr in slice.iter_mut() {
                *chr = b'0';
            }
        }
    }
}