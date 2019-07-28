use glitchconsole::options::{TomlProcessor, MutConfig, MutOptionVal};
use glitchconsole::loaders::Loader;
use glitchconsole::mutation::Mutation;

use glitchup_derive::MutConfig;

use serde::Deserialize;
use memmap::MmapMut;

use std::collections::HashMap;

// load file from config

#[derive(Debug, Deserialize, MutConfig)]
struct MainConfig {
    iterations: isize,
    chunksize: Vec<isize>, // A range of chunksizes.
    pub datalen: isize,

}

struct NullConfig {
    
}