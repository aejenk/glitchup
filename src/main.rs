mod mutations;
mod options;
mod loaders;
mod benders;

use options::TomlProcessor as Toml;
use options::{MutConfig, MutOptionVal};
use mutations::*;
use loaders::Loader;
use crate::benders::BasicBender;

use serde::Deserialize;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    // Load option file as a configuration
    let mut bender = BasicBender::new(String::from("options.toml"));

    // Initialise mutation
    let mut basic_mut = BasicMutation::default();

    // Copy foo.txt into foo2.txt and memory map
    bender.init_file(
        String::from("foo.txt"),
        Some(String::from("foo2.txt"))
    )?;

    // Mutate the memory map according to the loaded configuration
    bender.mutate(&mut Box::new(basic_mut));

    Ok(())
}
