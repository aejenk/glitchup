mod mutations;
mod options;
mod loaders;
mod benders;

use mutations::basic_mutation::BasicMutation;
use crate::benders::BasicBender;

fn main() -> std::io::Result<()> {
    let basic_mut = &mut Box::new(BasicMutation::default());

    // Mutate the memory map according to the loaded configuration
    let mut bender = BasicBender::new("options.toml", "foo.txt", None);

    bender
        .mutate(basic_mut)
        .mutate(basic_mut)
        .mutate(basic_mut)
        .mutate(basic_mut);

    println!("{:#?}", bender);

    Ok(())
}
