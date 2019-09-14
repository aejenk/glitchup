//! A main function. Currently doesn't have anything since work on 
//! a databender hasn't started yet.

mod benders;
use benders::KaBender;

use rayon::prelude::*;

mod mutations;

fn main() {
    // Initialises a bender with a configuration file.
    let bender = KaBender::new("Options.toml", String::from(""));

    // Retrieves some options from the configuration.
    let loops = bender.config.times.clone().unwrap_or(1);

    (0..loops).into_par_iter().for_each(|i| {
        let bender = KaBender::new("Options.toml", i.to_string());
        bender.run();
    });
}