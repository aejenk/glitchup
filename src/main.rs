//! A main function. Currently doesn't have anything since work on 
//! a databender hasn't started yet.

mod benders;
use benders::KaBender;

mod configuration;
use configuration::Configuration;

mod mutations;
mod loaders;
mod mutation;

use rayon::prelude::*;

fn main() {
    // Initialises the configuration for the application.
    let conf = match Configuration::from_file("Options.toml") {
        Err(msg) => {
            eprintln!("{}", msg);
            return;
        },
        Ok(conf) => conf,
    };

    // Retrieves some options from the configuration.
    let loops = conf.get("times")
        .and_then(|times| times.as_int())
        .unwrap_or(&300);

    (0..*loops).into_par_iter().for_each(|i| {
        let bender = KaBender::new(&conf, i.to_string());
        bender.run();
    });
}