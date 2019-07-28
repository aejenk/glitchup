use glitchconsole::options::{TomlProcessor, MutConfig, MutOptionVal};
use glitchconsole::loaders::Loader;
use glitchconsole::mutation::Mutation;

use glitchup_derive::MutConfig;

use serde::Deserialize;
use memmap::MmapMut;

use std::collections::HashMap;

/// The main configuration of the bender.
/// 
/// Represents the entire TOML options file.
#[derive(Debug, Deserialize, MutConfig)]
#[allow(unused_attributes)] // pops up a warning for custom attributes apparently.
pub struct MainConfig {
    #[ignore]
    inputfile : String, // Input file.
    #[ignore]
    outputfile : Option<String>, // Manually setting the output file.
    #[ignore]
    pub times : Option<isize>,
    iterations: Vec<isize>, // How many iteration every "mutate" does
    chunksize: Vec<isize>, // A range of chunksizes.
    #[ignore]
    pub mutations: Vec<Vec<String>>,
    loop_mut: LoopConfig
}

#[derive(Debug, Deserialize, MutConfig)]
pub struct LoopConfig {
    loops: Vec<isize>
}

/// A main controller of the databender.
/// 
/// Manages the file handling, data storage, and controls mutations.
pub struct KaBender {
    extension: String,
    output: String,
    data: MmapMut,
    pub config: MainConfig,
    log: Vec<String>
}

impl KaBender {
    /// Creates a new KaBender from the configuration.
    pub fn new(config_filename: &str) -> Self {
        println!("Initialising bender...");
        let mut new = KaBender {
            config : TomlProcessor::parse_toml_as_options(config_filename).unwrap(),
            extension : String::new(),
            output : String::new(),
            data : MmapMut::map_anon(1).unwrap(),
            log : Vec::new(),
        };

        new.init_file();
        new
    }

    /// Initialises the file.
    /// 
    /// Copies the input file to a temporary file, and memory maps the copy.
    /// Also initialises the filenames and extensions.
    fn init_file(&mut self) -> &mut Self {
        use std::path::Path;
        use std::ffi::OsStr;

        println!("Initialising file...");

        let input = &self.config.inputfile.clone();

        // Sets output name to custom name, or input if not specified.
        let output = &self.config.outputfile.clone()
            .unwrap_or(input.clone());

        let path = Path::new(&output);

        // Splits file into extension and filename.
        self.extension = String::from(path
            .extension()
            .and_then(OsStr::to_str)
            .unwrap()
            .clone());

        self.output = String::from(path
            .file_stem()
            .and_then(OsStr::to_str)
            .unwrap()
            .clone());

        // Memory maps the temporary output file.
        self.data = Loader::init_file_mut(
            input,
            format!("temp.{}", self.extension).as_str()
        ).unwrap();

        self
    }

    /// Configures the mutation passed with the Bender's configuration.
    pub fn configure_mutation(&mut self, mutation: &mut Box<dyn Mutation>) -> &mut Self {
        println!("Configuring mutation...");
        mutation.configure(Box::new(&self.config));
        self
    }

    /// Performs the mutation.
    /// 
    /// Also adds the mutation to the log.
    pub fn mutate_with(&mut self, mutation: &mut Box<dyn Mutation>) -> &mut Self {
        println!("Mutating data...");
        mutation.mutate(self.data.as_mut());
        self.log.push(mutation.to_string());
        self
    }

    /// Restarts the bender.
    /// 
    /// "Saves" the temporary file, and resets back to the original input file.
    /// Used to have multiple kinds of seperate mutations in one execution.
    /// 
    /// To chain mutations:
    /// ```
    /// .mutate(...)
    /// .mutate(...)
    /// ...
    /// ```
    /// 
    /// To save each mutation to a different file:
    /// ```
    /// .mutate(...)
    /// .restart()
    /// .mutate(...)
    /// .restart()
    /// ```
    pub fn restart(&mut self) -> &mut Self {
        // "Saves" file
        self.flush();

        // Memory maps another copy of the file
        self.init_file();

        // Resets the log
        self.log = Vec::new();

        self
    }

    /// "Saves" the file by renaming it from `temp.rs` to a generated output name.
    pub fn flush(&mut self){
        // Generates an output name
        let genoutput = format!("{name}__{muts}.{ext}",
            name = self.output.clone(),
            muts = self.log.join("---"),
            ext = self.extension.clone(),
        );

        println!("Renaming temporary file to {}", genoutput);

        // Renames temporary file to actual output name
        Loader::rename_file(
            format!("temp.{}", self.extension).as_str(),
            genoutput.as_str()
        ).unwrap();
    }
}
