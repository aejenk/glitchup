mod mutations;
mod opt_processor;
mod loaders;

use opt_processor::OptionProcessor as OPro;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct MainConfig {
    mutation : MutationConfig
}

#[derive(Debug, Deserialize)]
struct MutationConfig {
    min : usize,
    max : usize,
    chunksize : usize
}

fn main() {
    let options : MainConfig = OPro::parse(String::from("options.toml")).unwrap();

    println!("ParsedOptions: {:#?}", options)
}
