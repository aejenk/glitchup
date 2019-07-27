use glitchconsole::options::{TomlProcessor, MutConfig, MutOptionVal};
use glitchconsole::loaders::Loader;
use glitchconsole::mutation::Mutation;

use basic_mutation::BasicMutation;
use glitchup_derive::MutConfig;

use serde::Deserialize;
use memmap::MmapMut;

use std::collections::HashMap;

#[derive(Debug, Deserialize, MutConfig)]
struct MainConfig {
    mutation : MutationConfig,
}

#[derive(Debug, Deserialize, MutConfig)]
struct MutationConfig {
    min : Option<isize>,
    max : Option<isize>,
    chunksize : isize,
}

#[derive(Debug)]
pub struct BasicBender<'a> {
    filename: &'a str,
    extension: &'a str,
    output: &'a str,
    curr_iter: u32,
    data: MmapMut,
    config: MainConfig
}

impl<'a> BasicBender<'a> {
    pub fn new(config_filename: &str, input: &'a str, output: Option<&'a str>) -> Self {
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

    pub fn init_file(&mut self, input: &'a str, output: Option<&'a str>) -> &mut Self {

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

    pub fn mutate<T: Mutation>(&mut self, mutation: &mut Box<T>) -> &mut Self {
        // performs mutation
        mutation.mutate(
            self.data.as_mut(),
            Box::new(&self.config)
        );

        self
    }
}