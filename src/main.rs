//! A main function. Currently doesn't have anything since work on 
//! a databender hasn't started yet.

mod benders;
use benders::KaBender;

mod mutations;
use mutations::void::Void;

/// A do-nothin function that's sayin hello to you.
fn main() {
    let mutation = &mut Box::new(Void::default());

    let mut bender = KaBender::new("Options.toml");

    let loops = bender.config.loops.clone().unwrap_or(1);

    for _ in 0..loops {
        bender
            .configure_mutation(mutation)
            .mutate_with(mutation)
            .restart();
    }
}