use super::options::{TomlProcessor, MutConfig, MutOptionVal};
use super::mutations::*;
use super::loaders::Loader;

use serde::Deserialize;
use memmap::MmapMut;

use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct MainConfig {
    mutation : MutationConfig
}

#[derive(Debug, Deserialize)]
struct MutationConfig {
    min : isize,
    max : isize,
    chunksize : isize
}

impl MutConfig for MainConfig {
    fn to_hashmap(&self) -> HashMap<String, MutOptionVal> {
        use MutOptionVal::*;
        let mut map = HashMap::new();
        let mut muts = HashMap::new();

        muts.insert(String::from("min"), OInt(self.mutation.min));
        muts.insert(String::from("max"), OInt(self.mutation.max));
        muts.insert(String::from("chunksize"), OInt(self.mutation.chunksize));

        map.insert(String::from("mutation"), OMap(muts));
        map
    }
}

pub struct BasicBender {
    filename: String,
    extension: String,
    output: String,
    curr_iter: u32,
    data: Option<MmapMut>,
    config: MainConfig
}

impl BasicBender {
    pub fn new(config_filename: String) -> BasicBender {
        BasicBender {
            curr_iter : 1,
            config : TomlProcessor::parse_toml_as_options(config_filename).unwrap(),
            filename : String::new(),
            extension : String::new(),
            output : String::new(),
            data : None
        }
    }

    pub fn init_file(&mut self, input: String, output: Option<String>) -> std::io::Result<()> {

        // Set optional output
        let out = if let Some(name) = output {
            name
        }
        else {
            input.clone()
        };

        // Find index for extension
        let extindex = out.clone().rfind('.').unwrap_or(out.len());

        // Backup filenames
        self.filename = input.clone();
        self.extension = String::from(out.clone().get(extindex+1..out.len()).unwrap());
        self.output = String::from(out.clone().get(0..extindex).unwrap());

        // Load data
        self.data = Some(Loader::init_file_mut(
            input.clone(),
            format!("{}_iter={}.{}", self.output.clone(), self.curr_iter, self.extension.clone())
        )?);

        Ok(())
    }

    pub fn mutate<T: Mutation>(&mut self, mutation: &mut Box<T>){
        // verifies that the bender is in a valid state
        self.verify();

        // performs mutation
        mutation.mutate(
            &mut *self.data.as_mut().unwrap(),
            Box::new(&self.config)
        );
    }

    fn verify(&self) {
        if self.data.is_none() {
            panic!("No file was loaded. Have you called 'init_file'?");
        }
    }
}