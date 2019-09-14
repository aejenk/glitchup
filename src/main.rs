//! A main function. Currently doesn't have anything since work on 
//! a databender hasn't started yet.

mod benders;
use benders::KaBender;

mod mutations;

fn main() {
    // Initialises a bender with a configuration file.
    let bender = KaBender::new("Options.toml");

    // Retrieves some options from the configuration.
    let loops = bender.config.times.clone().unwrap_or(1);

    (0..loops).for_each(|_| {
        let bender = KaBender::new("Options.toml");
        bender.run();
    });
}