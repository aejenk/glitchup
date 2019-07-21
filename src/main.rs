mod mutations;
mod options;
mod loaders;
mod benders;

use mutations::*;
use crate::benders::BasicBender;


fn main() -> std::io::Result<()> {
    // Load option file as a configuration
    let mut bender = BasicBender::new(String::from("options.toml"));

    // Initialise mutation
    let basic_mut = BasicMutation::default();

    // Initialise input and output file names for bender
    bender.init_file(
        String::from("foo.txt"),
        Some(String::from("foo2.txt"))
    )?;

    // Mutate the memory map according to the loaded configuration
    bender.mutate(&mut Box::new(basic_mut));

    Ok(())
}
