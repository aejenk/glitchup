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
    min : Option<isize>,
    max : Option<isize>,
    chunksize : isize
}

impl MutConfig for MainConfig {
    fn to_hashmap(&self) -> HashMap<String, MutOptionVal> {
        use MutOptionVal::*;
        let mut map = HashMap::new();
        let mut muts = HashMap::new();

        let min = self.mutation.min.map_or(ONone(), |n| OInt(n));
        let max = self.mutation.max.map_or(ONone(), |n| OInt(n));

        muts.insert(String::from("min"), min);
        muts.insert(String::from("max"), max);
        muts.insert(String::from("chunksize"), OInt(self.mutation.chunksize));

        map.insert(String::from("mutation"), OMap(muts));
        map
    }
}

pub struct BasicBender<'a> {
    filename: &'a str,
    extension: &'a str,
    output: &'a str,
    curr_iter: u32,
    data: MmapMut,
    config: MainConfig
}

impl<'a> BasicBender<'a> {
    pub fn new(config_filename: &str, input: &'a str, output: Option<&'a str>) -> BasicBender<'a> {
        let mut return_bender = BasicBender {
            curr_iter : 1,
            config : TomlProcessor::parse_toml_as_options(config_filename).unwrap(),
            filename : "",
            extension: "",
            output: "",
            data : MmapMut::map_anon(1).unwrap()
        };

        return_bender.init_file(input, output);

        return_bender
    }

    pub fn init_file(&mut self, input: &'a str, output: Option<&'a str>) -> &mut BasicBender<'a> {

        // Set optional output
        let out = output.unwrap_or(input);

        // Find index for extension
        let extindex = out.rfind('.').unwrap_or(out.len());

        // Backup filenames
        // Hint: For extensions, you can use this: 
        //      https://stackoverflow.com/questions/45291832/extracting-a-file-extension-from-a-given-path-in-rust-idiomatically
        self.filename = input;
        self.extension = out.get(extindex+1..out.len()).unwrap();
        self.output = out.get(0..extindex).unwrap();

        // Load data
        self.data = Loader::init_file_mut(
            input,
            format!("{}_iter={}.{}", self.output.clone(), self.curr_iter, self.extension.clone()).as_str()
        ).unwrap();

        self
    }

    pub fn mutate<T: Mutation>(&mut self, mutation: &mut Box<T>) -> &mut BasicBender<'a> {
        // performs mutation
        mutation.mutate(
            self.data.as_mut(),
            Box::new(&self.config)
        );

        self
    }
}