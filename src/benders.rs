use glitchconsole::loaders::Loader;
use glitchconsole::mutation::Mutation;

use memmap::MmapMut;

use super::mutations::{
    void::Void, chaos::Chaos, loops::Loops, reverse::Reverse,
    shift::Shift, shuffle::Shuffle, swap::Swap,
    increase::Increase, gradient::Gradient, multiply::Multiply,
    compress::Compress,
};

use super::configuration::Configuration;

use std::collections::HashMap;

use rayon::prelude::*;

type Mut = Box<dyn Mutation + Send + Sync>;
type Muts = Vec<Mut>;

/// A main controller of the databender.
/// 
/// Manages the file handling, data storage, and controls mutations.
#[derive(Debug)]
pub struct KaBender<'a> {
    pub seed: String,
    outdir: String,
    extension: String,
    output: String,
    mutmap: HashMap<String, Mut>,
    pub config: &'a Configuration,
}

impl<'a> KaBender<'a> {
    /// Creates a new KaBender from the configuration.
    pub fn new(configuration: &'a Configuration, seed: String) -> Self {
        println!("Initialising bender...");
        let mut new : KaBender = KaBender {
            seed: seed,
            config : configuration,
            extension : String::new(),
            output : String::new(),
            outdir : String::new(),
            mutmap: HashMap::new(),
        };

        new.setup_mutations();
        new.setup_file_data();
        new
    }

    /// Executes the bender.
    /// 
    /// Performs all mutation combinations using the configuration loaded.
    pub fn run(mut self) {
        // Generates a file for each list of mutations
        let filelist : Vec<MmapMut> = self.init_file_n(self.config.mutations.len());

        // Retrieves all mutations from hashmap using the file.
        let mutations : Vec<Muts> = self.config.mutations.iter().map(|combo| {
            combo.iter().map(|mut_str| {
                self.mutmap.get(mut_str).cloned().unwrap()
            }).collect()
        }).collect();

        // Pairs each memory-mapped file to a list of mutations.
        let mut mut_map : Vec<(Muts, MmapMut)> = mutations
            .into_iter()
            .zip(filelist
                .into_iter())
            .collect();

        // Performs the mutations in parallel
        mut_map
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, (mutation_combo, map))| {
                let mut log = Vec::new();

                for mutation in mutation_combo {
                    mutation.mutate(map);
                    log.push(mutation.to_string());
                }

                self.flush(index, log);
            });
    }

    /// Initialises multiple memory mapped copies of a file.
    /// 
    /// * `n` - Number of files to initialize
    fn init_file_n(&mut self, n: usize) -> Vec<MmapMut> {

        println!("Initialising file...");

        (0..n)
            .into_par_iter()
            .map(|index| {
                Loader::init_file_mut(
                    &self.config.inputfile.clone(),
                    format!("{}temp{}SEED={}.{}", self.outdir, index, self.seed, self.extension).as_str()
                ).unwrap()
            }).collect()
    }

    /// Sets up the data of the file, such as the input, output, extension, and path.
    fn setup_file_data(&mut self) {
        use std::path::Path;
        use std::ffi::OsStr;

        let input = &self.config.inputfile.clone();

        // Sets output name to custom name, or input if not specified.
        let output = &self.config.outputfile.clone()
            .unwrap_or(input.clone());

        let path = Path::new(&output);

        // Extracts the extension from the filename
        self.extension = String::from(path
            .extension()
            .and_then(OsStr::to_str)
            .unwrap()
            .clone());

        // Extracts the output directory.
        // In X/Y.../Z.EXT, this extracts X/Y.../
        self.outdir = path.parent().and_then(Path::to_str).map_or(String::new(), |text| {
            if text == "" {
                String::new()
            } else {
                format!("{}/", text)
            }
        });

        // Represents the full path, apart from the extension.
        // In X/Y/.../Z.EXT, this extracts X/Y.../Z
        self.output = format!(
            "{}{}",
            self.outdir,
            path.file_stem().and_then(OsStr::to_str).unwrap().clone(),
        );
    }


    /// Setup the internal mutations for the Bender.
    /// In order to add your own mutation, you would need to include it here, otherwise it wouldn't be used.
    fn setup_mutations(&mut self) {
        fn generate_map(muts: Vec<(&'static str, Mut)>) -> HashMap<String, Mut> {
            muts.into_iter().map(|tuple| (String::from(tuple.0), tuple.1)).collect()
        }

        let mutmap = generate_map(vec![
            ("Void"     , Box::new(Void::default())),
            ("Chaos"    , Box::new(Chaos::default())),
            ("Loops"    , Box::new(Loops::default())),
            ("Reverse"  , Box::new(Reverse::default())),
            ("Shift"    , Box::new(Shift::default())),
            ("Shuffle"  , Box::new(Shuffle::default())),
            ("Swap"     , Box::new(Swap::default())),
            ("Increase" , Box::new(Increase::default())),
            ("Gradient" , Box::new(Gradient::default())),
            ("Multiply" , Box::new(Multiply::default())),
            ("Compress" , Box::new(Compress::default())),
        ]);

        self.mutmap = mutmap
            .into_iter()
            .map(|mut mutation| {
                mutation.1.configure(Box::new(self.config));
                mutation
            })
            .collect();
    }

    /// Renames the temporary file that was mutated to its supposed output file.
    /// 
    /// * `iter` - The iteration. Used to rename the right mutated file.
    /// * `log` - The log of mutations applied to the file. Used to embed mutation data into the filename itself.
    fn flush(&self, iter: usize, log: Vec<String>){
        let mut temp_muts = log.join("---");
        if temp_muts.len() > 200 {
            temp_muts.truncate(200);
            println!("Truncating mutation name due to length...");
        }

        // Generates an output name
        let genoutput = format!("{name}__{muts}.{ext}",
            name = self.output.clone(),
            muts = temp_muts,
            ext = self.extension.clone(),
        );

        println!("Renaming temporary file to {}", genoutput);

        println!("{}temp{}SEED={}.{}", self.outdir, iter, self.seed, self.extension);

        // Renames temporary file to actual output name
        Loader::rename_file(
            format!("{}temp{}SEED={}.{}", self.outdir, iter, self.seed, self.extension).as_str(),
            genoutput.as_str()
        ).unwrap();
    }
}
