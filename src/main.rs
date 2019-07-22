mod mutations;
mod options;
mod loaders;
mod benders;

use mutations::*;
use crate::benders::BasicBender;


fn main() -> std::io::Result<()> {
    // Initialise mutation
    let basic_mut = BasicMutation::default();

    // Mutate the memory map according to the loaded configuration
    BasicBender::new("options.toml", "foo.txt", None)
        .mutate(&mut Box::new(basic_mut));

    Ok(())
}
