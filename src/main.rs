//! A main function. Currently doesn't have anything since work on 
//! a databender hasn't started yet.

mod benders;
use benders::KaBender;

mod configuration;
use configuration::Configuration;

use rayon::prelude::*;

mod mutations;

fn main() {
    // Initialises the configuration for the application.
    let conf = match Configuration::new("Options.toml") {
        Err(msg) => {
            eprintln!("{}", msg);
            return;
        },
        Ok(conf) => conf,
    };

    // Retrieves some options from the configuration.
    let loops = conf.times.clone().unwrap_or(1);

    (0..loops).into_par_iter().for_each(|i| {
        let bender = KaBender::new(&conf, i.to_string());
        bender.run();
    });
}