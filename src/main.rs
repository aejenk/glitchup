mod mutations;
mod options;
mod loaders;

use options::TomlProcessor as Toml;
use options::{MutConfig, MutOptionVal};
use mutations::*;
use loaders::Loader;

use serde::Deserialize;
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

fn main() -> std::io::Result<()> {
    // Load option file as a configuration
    let options : MainConfig = Toml::parse_toml_as_options(String::from("options.toml")).unwrap();

    // Initialise mutation
    let mut basic_mut = BasicMutation::default();

    // Copy foo.txt into foo2.txt and memory map
    let mut mmap = Loader::init_file_mut(String::from("foo.txt"), String::from("foo2.txt"))?;

    // Mutate the memory map according to the loaded configuration
    basic_mut.mutate(&mut *mmap, Box::new(options));

    Ok(())
}
