mod mutations;
mod options;
mod loaders;
mod benders;

use mutations::basic_mutation::BasicMutation;
use crate::benders::BasicBender;


fn main() -> std::io::Result<()> {
    // Initialise mutation
    let basic_mut = &mut Box::new(BasicMutation::default());

    // Mutate the memory map according to the loaded configuration
    BasicBender::new("options.toml", "foo.txt", None)
        .mutate(basic_mut)
        .mutate(basic_mut)
        .mutate(basic_mut)
        .mutate(basic_mut)
        .mutate(basic_mut)
        .mutate(basic_mut)
        .mutate(basic_mut);

    Ok(())
}
