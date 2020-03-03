//! A main function. Currently doesn't have anything since work on 
//! a databender hasn't started yet.

mod benders;
mod mutations;
mod loaders;
mod configuration;

use benders::KaBender;
use configuration::Configuration;

use rayon::prelude::*;

fn main() {
    // Initialises the configuration for the application.
    let conf = Configuration::from_file("Options.toml");

    conf.verify_config();

    // Retrieves some options from the configuration.
    let loops = conf.get("times")
        .and_then(|times| times.as_int())
        .unwrap_or(&1);

    (0..*loops).into_par_iter().for_each(|i| {
        let bender = KaBender::new(&conf, i.to_string());
        bender.run();
    });
}