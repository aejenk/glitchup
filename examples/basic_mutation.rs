use glitchconsole::mutation::{Mutation};
use glitchconsole::options::{MutConfig};
use std::fmt::{Display, Formatter, Error};

use rand::Rng;

#[derive(Default)]
pub struct BasicMutation {
    min : usize,
    max : usize,
    chunk_size : usize
}

impl BasicMutation {
}

impl Display for BasicMutation {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "min={}_max={}_csize={}", self.min, self.max, self.chunk_size)
    }
}

impl Mutation for BasicMutation {
    fn configure(&mut self, config: Box<&MutConfig>) {
        // to avoid verbosity
        use glitchconsole::options::MutOptionVal::*;

        let mutopts = &config.to_hashmap();

        let datalen = if let OInt(size) = &mutopts["datalen"] {
            *size as usize
        } else {
            panic!("Somehow, the data loaded has no length. Please contact the dev.")
        };

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
            if *omax as usize > datalen - self.chunk_size {
                self.max = datalen - self.chunk_size;
            }
            else {
                self.max = *omax as usize;
            }
        }
        else {
            self.max = datalen - self.chunk_size;
        }
    }

    fn mutate(&mut self, data: &mut [u8]) {
        // random number generator
        let mut rng = rand::thread_rng();

        let index: usize = rng.gen_range(self.min, self.max);

        if let Some(slice) = data.get_mut(index..self.chunk_size+index) {
            for chr in slice.iter_mut() {
                *chr = b'0';
            }
        }
    }
}