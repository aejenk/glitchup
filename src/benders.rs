use glitchconsole::options::{TomlProcessor, MutConfig, MutOptionVal};
use glitchconsole::loaders::Loader;
use glitchconsole::mutation::Mutation;

use glitchup_derive::MutConfig;

use serde::Deserialize;
use memmap::MmapMut;

use super::mutations::{
    void::Void, chaos::Chaos, loops::Loops, reverse::Reverse,
    shift::Shift, shuffle::Shuffle, swap::Swap,
    increase::Increase, gradient::Gradient
};

use std::collections::HashMap;

use rayon::prelude::*;

type Mut = Box<dyn Mutation + Send + Sync>;
type Muts = Vec<Mut>;

/// The main configuration of the bender.
/// 
/// Represents the entire TOML options file.
#[derive(Debug, Deserialize, MutConfig)]
#[allow(unused_attributes, non_snake_case)] // pops up a warning for custom attributes apparently.
pub struct MainConfig {
    /// The name of the input file.
    /// Is required.
    #[ignore]
    inputfile : String,

    /// The name of the output file.
    /// If not passed, the name of the input file is used.
    #[ignore]
    outputfile : Option<String>,

    /// The number of times to repeat the program.
    /// If not passed, the value defaults to 1.
    #[ignore]
    pub times : Option<isize>,

    /// A range of iterations.
    /// Specifies how many times each mutation is applied.
    /// A global option to be set for all relevant mutations.
    iterations: Vec<isize>,

    /// A range of chunksizes.
    /// Specifies the size of each chunk of *bytes* to mutate.
    /// A global option to be set for all relevant mutations.
    chunksize: Vec<isize>,

    /// A list of mutations to be used. 
    #[ignore]
    pub mutations: Vec<Vec<String>>,

    // Mutation configurations.
    VoidConfig: Option<VoidConfig>,
    ChaosConfig: Option<ChaosConfig>,
    LoopConfig: Option<LoopConfig>,
    ReverseConfig: Option<ReverseConfig>,
    ShiftConfig: Option<ShiftConfig>,
    ShuffleConfig: Option<ShuffleConfig>,
    SwapConfig: Option<SwapConfig>,
    IncreaseConfig: Option<IncreaseConfig>,
    GradientConfig: Option<GradientConfig>
}

#[derive(Debug, Deserialize, Clone, MutConfig)]
pub struct VoidConfig {
    iterations: Option<Vec<isize>>,
    chunksize: Option<Vec<isize>>,
}

#[derive(Debug, Deserialize, Clone, MutConfig)]
pub struct ChaosConfig {
    iterations: Option<Vec<isize>>,
    chunksize: Option<Vec<isize>>,
}

#[derive(Debug, Deserialize, Clone, MutConfig)]
pub struct LoopConfig {
    iterations: Option<Vec<isize>>,
    chunksize: Option<Vec<isize>>,
    loops: Option<Vec<isize>>
}

#[derive(Debug, Deserialize, Clone, MutConfig)]
pub struct ReverseConfig {
    iterations: Option<Vec<isize>>,
    chunksize: Option<Vec<isize>>,
}

#[derive(Debug, Deserialize, Clone, MutConfig)]
pub struct ShiftConfig {
    iterations: Option<Vec<isize>>,
    chunksize: Option<Vec<isize>>,
}

#[derive(Debug, Deserialize, Clone, MutConfig)]
pub struct ShuffleConfig {
    iterations: Option<Vec<isize>>,
    chunksize: Option<Vec<isize>>,
}

#[derive(Debug, Deserialize, Clone, MutConfig)]
pub struct SwapConfig {
    iterations: Option<Vec<isize>>,
    chunksize: Option<Vec<isize>>,
}

#[derive(Debug, Deserialize, Clone, MutConfig)]
pub struct IncreaseConfig {
    iterations: Option<Vec<isize>>,
    chunksize: Option<Vec<isize>>,
    increase_by: Option<Vec<isize>>
}

#[derive(Debug, Deserialize, Clone, MutConfig)]
pub struct GradientConfig {
    iterations: Option<Vec<isize>>,
    chunksize: Option<Vec<isize>>,
    accelerate_by: Option<Vec<isize>>,
    accelerate_in: Option<Vec<isize>> 
}



/// A main controller of the databender.
/// 
/// Manages the file handling, data storage, and controls mutations.
#[derive(Debug)]
pub struct KaBender {
    pub seed: String,
    outdir: String,
    extension: String,
    output: String,
    mutmap: HashMap<String, Mut>,
    pub config: MainConfig,
}

impl KaBender {
    /// Creates a new KaBender from the configuration.
    pub fn new(config_filename: &str, seed: String) -> Self {
        println!("Initialising bender...");
        let mut new = KaBender {
            seed: seed,
            config : TomlProcessor::parse_toml_as_options(config_filename).unwrap(),
            extension : String::new(),
            output : String::new(),
            outdir : String::new(),
            mutmap: HashMap::new(),
        };

        new.setup_config();
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

    /// Sets up the configurations.
    /// 
    /// Is a huge function due to multiple repeated boilerplate code.
    /// In the future, there might be macros etc. to improve the style.
    /// 
    /// To add your own mutation, you would need to set up its configuration in this function.
    fn setup_config(&mut self) {
        let muts_passed = self.config.mutations.concat();

        self.config.iterations = verify_num_option(&self.config.iterations, "iterations", "globally");
        self.config.chunksize = verify_num_option(&self.config.chunksize, "chunksize", "globally");

        let void_exists = muts_passed.contains(&String::from("Void"));
        let chaos_exists = muts_passed.contains(&String::from("Chaos"));
        let loops_exists = muts_passed.contains(&String::from("Loops"));
        let reverse_exists = muts_passed.contains(&String::from("Reverse"));
        let shift_exists = muts_passed.contains(&String::from("Shift"));
        let shuffle_exists = muts_passed.contains(&String::from("Shuffle"));
        let swap_exists = muts_passed.contains(&String::from("Swap"));
        let increase_exists = muts_passed.contains(&String::from("Increase"));
        let gradient_exists = muts_passed.contains(&String::from("Gradient"));

        // If mutation not included, reset it to None.
        if !void_exists         {self.config.VoidConfig         = None};
        if !chaos_exists        {self.config.ChaosConfig        = None};
        if !loops_exists        {self.config.LoopConfig         = None};
        if !reverse_exists      {self.config.ReverseConfig      = None};
        if !shift_exists        {self.config.ShiftConfig        = None};
        if !shuffle_exists      {self.config.ShuffleConfig      = None};
        if !swap_exists         {self.config.SwapConfig         = None};
        if !increase_exists     {self.config.IncreaseConfig     = None};
        if !gradient_exists     {self.config.GradientConfig     = None};

        // VoidConfig setup
        if self.config.VoidConfig.is_none() && void_exists {
            self.config.VoidConfig = Some(VoidConfig {
                iterations: Some(self.config.iterations.clone()),
                chunksize: Some(self.config.chunksize.clone())
            });
        }
        else if let (Some(x), true) = (&mut self.config.VoidConfig, void_exists) {
            x.chunksize = if let Some(ch) = &x.chunksize
                {Some(verify_num_option(&ch, "chunksize", "under '[VoidConfig]'"))}
                else {Some(self.config.chunksize.clone())};
            
            x.iterations = if let Some(ch) = &x.iterations
                {Some(verify_num_option(&ch, "iterations", "under '[VoidConfig]'"))}
                else {Some(self.config.iterations.clone())};
        };

        // ChaosConfig setup
        if self.config.ChaosConfig.is_none() && chaos_exists {
            self.config.ChaosConfig = Some(ChaosConfig {
                iterations: Some(self.config.iterations.clone()),
                chunksize: Some(self.config.chunksize.clone())
            });
        }
        else if let (Some(x), true) = (&mut self.config.ChaosConfig, chaos_exists) {
            x.chunksize = if let Some(ch) = &x.chunksize
                {Some(verify_num_option(&ch, "chunksize", "under '[ChaosConfig]'"))}
                else {Some(self.config.chunksize.clone())};
            
            x.iterations = if let Some(ch) = &x.iterations
                {Some(verify_num_option(&ch, "iterations", "under '[ChaosConfig]'"))}
                else {Some(self.config.iterations.clone())};
        };

        // LoopConfig setup
        if self.config.LoopConfig.is_none() && loops_exists {
            let example = r#"
                [LoopConfig]
                loops = [1,2]
            "#;

            panic!("You have added a 'Loops' mutation, but haven't passed its options.
                    \nSpecifically, the 'loops' option needs to be passed under '[LoopConfig]'.
                    \nThe following is an example:
                    \n{}", example);
        }
        else if let (Some(x), true) = (&mut self.config.LoopConfig, loops_exists) {
            x.chunksize = if let Some(ch) = &x.chunksize
                {Some(verify_num_option(&ch, "chunksize", "under '[LoopConfig]'"))}
                else {Some(self.config.chunksize.clone())};
            
            x.iterations = if let Some(ch) = &x.iterations
                {Some(verify_num_option(&ch, "iterations", "under '[LoopConfig]'"))}
                else {Some(self.config.iterations.clone())};

            if x.loops.is_none() {
                panic!("You have added a 'Loops' mutation, but haven't passed the 'loops' option.
                        \nFor example: 'loops = [1,5]'");
            }
            else if let Some(l) = &x.loops {
                x.loops = Some(verify_num_option(&l, "loops", "under '[LoopConfig]'"));
            };
        };

        // ReverseConfig setup
        if self.config.ReverseConfig.is_none() && reverse_exists {
            self.config.ReverseConfig = Some(ReverseConfig {
                iterations: Some(self.config.iterations.clone()),
                chunksize: Some(self.config.chunksize.clone())
            });
        }
        else if let (Some(x), true) = (&mut self.config.ReverseConfig, reverse_exists) {
            x.chunksize = if let Some(ch) = &x.chunksize
                {Some(verify_num_option(&ch, "chunksize", "under '[ReverseConfig]'"))}
                else {Some(self.config.chunksize.clone())};
            
            x.iterations = if let Some(ch) = &x.iterations
                {Some(verify_num_option(&ch, "iterations", "under '[ReverseConfig]'"))}
                else {Some(self.config.iterations.clone())};
        };

        // ShiftConfig setup
        if self.config.ShiftConfig.is_none() && shift_exists {
            self.config.ShiftConfig = Some(ShiftConfig {
                iterations: Some(self.config.iterations.clone()),
                chunksize: Some(self.config.chunksize.clone())
            });
        }
        else if let (Some(x), true) = (&mut self.config.ShiftConfig, shift_exists) {
            x.chunksize = if let Some(ch) = &x.chunksize
                {Some(verify_num_option(&ch, "chunksize", "under '[ShiftConfig]'"))}
                else {Some(self.config.chunksize.clone())};
            
            x.iterations = if let Some(ch) = &x.iterations
                {Some(verify_num_option(&ch, "iterations", "under '[ShiftConfig]'"))}
                else {Some(self.config.iterations.clone())};
        };

        // ShuffleConfig setup
        if self.config.ShuffleConfig.is_none() && shuffle_exists {
            self.config.ShuffleConfig = Some(ShuffleConfig {
                iterations: Some(self.config.iterations.clone()),
                chunksize: Some(self.config.chunksize.clone())
            });
        }
        else if let (Some(x), true) = (&mut self.config.ShuffleConfig, shuffle_exists) {
            x.chunksize = if let Some(ch) = &x.chunksize
                {Some(verify_num_option(&ch, "chunksize", "under '[ShuffleConfig]'"))}
                else {Some(self.config.chunksize.clone())};
            
            x.iterations = if let Some(ch) = &x.iterations
                {Some(verify_num_option(&ch, "iterations", "under '[ShuffleConfig]'"))}
                else {Some(self.config.iterations.clone())};
        };

        // SwapConfig setup
        if self.config.SwapConfig.is_none() && swap_exists {
            self.config.SwapConfig = Some(SwapConfig {
                iterations: Some(self.config.iterations.clone()),
                chunksize: Some(self.config.chunksize.clone())
            });
        }
        else if let (Some(x), true) = (&mut self.config.SwapConfig, swap_exists) {
            x.chunksize = if let Some(ch) = &x.chunksize
                {Some(verify_num_option(&ch, "chunksize", "under '[SwapConfig]'"))}
                else {Some(self.config.chunksize.clone())};
            
            x.iterations = if let Some(ch) = &x.iterations
                {Some(verify_num_option(&ch, "iterations", "under '[SwapConfig]'"))}
                else {Some(self.config.iterations.clone())};
        };

        // IncreaseConfig setup
        if self.config.IncreaseConfig.is_none() && increase_exists {
            let example = r#"
                [IncreaseConfig]
                increase_by = [1,2]
            "#;

            panic!("You have added a 'IncreaseConfig' mutation, but haven't passed its options.
                    \nSpecifically, the 'increase_by' option needs to be passed under '[IncreaseConfig]'.
                    \nThe following is an example:
                    \n{}", example);
        }
        else if let (Some(x), true) = (&mut self.config.IncreaseConfig, increase_exists) {
            x.chunksize = if let Some(ch) = &x.chunksize
                {Some(verify_num_option(&ch, "chunksize", "under '[IncreaseConfig]'"))}
                else {Some(self.config.chunksize.clone())};
            
            x.iterations = if let Some(ch) = &x.iterations
                {Some(verify_num_option(&ch, "iterations", "under '[IncreaseConfig]'"))}
                else {Some(self.config.iterations.clone())};

            if x.increase_by.is_none() {
                panic!("You have added a 'Increase' mutation, but haven't passed the 'increase_by' option.
                        \nFor example: 'increase_by = [1,5]'");
            }
            else if let Some(l) = &x.increase_by {
                x.increase_by = Some(verify_num_option(&l, "increase_by", "under '[LoopConfig]'"));
            };
        };

        // GradientConfig setup
        if self.config.GradientConfig.is_none() && gradient_exists {
            let example = r#"
                [GradientConfig]
                accelerate_by = [1,2]
                accelerate_in = [1,2]
            "#;

            panic!("You have added a 'GradientConfig' mutation, but haven't passed its options.
                    \nSpecifically, the 'accelerate_by' and 'accelerate_in'
                    \noption needs to be passed under '[LoopConfig]'.
                    \nThe following is an example:
                    \n{}", example);
        }
        else if let (Some(x), true) = (&mut self.config.GradientConfig, gradient_exists) {
            x.chunksize = if let Some(ch) = &x.chunksize
                {Some(verify_num_option(&ch, "chunksize", "under '[GradientConfig]'"))}
                else {Some(self.config.chunksize.clone())};
            
            x.iterations = if let Some(ch) = &x.iterations
                {Some(verify_num_option(&ch, "iterations", "under '[GradientConfig]'"))}
                else {Some(self.config.iterations.clone())};

            if x.accelerate_by.is_none() {
                panic!("You have added a 'Gradient' mutation, but haven't passed the 'accelerate_by' option.
                        \nFor example: 'accelerate_by = [1,5]'");
            }
            else if let Some(l) = &x.accelerate_by {
                x.accelerate_by = Some(verify_num_option(&l, "accelerate_by", "under '[GradientConfig]'"));
            };

            if x.accelerate_in.is_none() {
                panic!("You have added a 'Gradient' mutation, but haven't passed the 'accelerate_in' option.
                        \nFor example: 'accelerate_in = [1,5]'");
            }
            else if let Some(l) = &x.accelerate_in {
                x.accelerate_in = Some(verify_num_option(&l, "accelerate_in", "under '[GradientConfig]'"));
            };
        };

        fn verify_num_option(v : &Vec<isize>, name: &str, location: &str) -> Vec<isize> {
            let len = v.len();
            if len != 1 && len != 2 {
                panic!("You have passed an invalid '{0}' option {2}. It needs to be a vector with 1
                        \nor 2 elements. [exact = '{0} = [3]', range = '{0} = [1,5]'
                        \nYour option: {1:?}", name, v, location);
            }
            else if len == 1 {
                return vec![v[0], v[0]+1];
            }
            else {
                if v[0] > v[1] {
                    return vec![v[1], v[0]+1];
                }
                else {
                    return vec![v[0], v[1]+1];
                }
            }
        }
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
        ]);

        self.mutmap = mutmap
            .into_iter()
            .map(|mut mutation| {
                mutation.1.configure(Box::new(&self.config));
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
