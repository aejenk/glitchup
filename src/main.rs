//! A main function. Currently doesn't have anything since work on 
//! a databender hasn't started yet.

mod benders;
use benders::KaBender;

mod mutations;
use mutations::{
    void::Void, chaos::Chaos, loops::Loops, reverse::Reverse,
    shift::Shift, shuffle::Shuffle, swap::Swap,
    increase::Increase, gradient::Gradient
};

use std::collections::HashMap;
use glitchconsole::mutation::Mutation;

fn main() {

    // Initialises a bender with a configuration file.
    let mut bender = KaBender::new("Options.toml");

    // Retrieves some options from the configuration.
    let loops = bender.config.times.clone().unwrap_or(1);
    let muts = bender.config.mutations.clone();

    // Sets up the available mutations in the app
    let mut mapmuts : HashMap<String, Box<dyn Mutation>> = HashMap::new();
    mapmuts.insert(String::from("Void")     , Box::new(Void::default()));
    mapmuts.insert(String::from("Chaos")    , Box::new(Chaos::default()));
    mapmuts.insert(String::from("Loops")    , Box::new(Loops::default()));
    mapmuts.insert(String::from("Reverse")  , Box::new(Reverse::default()));
    mapmuts.insert(String::from("Shift")    , Box::new(Shift::default()));
    mapmuts.insert(String::from("Shuffle")  , Box::new(Shuffle::default()));
    mapmuts.insert(String::from("Swap")     , Box::new(Swap::default()));
    mapmuts.insert(String::from("Increase") , Box::new(Increase::default()));
    mapmuts.insert(String::from("Gradient") , Box::new(Gradient::default()));

    // Configures all included mutations
    for (_ , v) in mapmuts.iter_mut() {
        bender.configure_mutation(v);
    }

    // Executes databending
    for _ in 0..loops {
        for mutcombo in &muts {
            for mutation in mutcombo {
                let mt = mapmuts.get_mut(mutation);
                if let None = mt {
                    panic!("{} is not a valid mutation name.", mutation);
                };
                println!("Mutating with {}...", mutation);
                bender.mutate_with(&mut mt.unwrap());
            }
            println!("Restarting bender...");
            bender.restart();
        }
    }

    bender.flush();
}