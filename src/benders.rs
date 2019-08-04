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
    outdir: String,
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
            outdir : String::new(),
            data : MmapMut::map_anon(1).unwrap(),
            log : Vec::new(),
        };

        new.setup_config();
        new.init_file();
        new
    }

    /// Sets up the configurations.
    /// 
    /// Is a huge function due to multiple repeated boilerplate code.
    /// In the future, there might be macros etc. to improve the style.
    fn setup_config(&mut self) {
        let muts_passed = self.config.mutations.concat();

        self.config.iterations = verify_num_option(&self.config.iterations, "iterations", "globally");
        self.config.chunksize = verify_num_option(&self.config.iterations, "chunksize", "globally");

        let void_exists = muts_passed.contains(&String::from("Void"));
        let chaos_exists = muts_passed.contains(&String::from("Chaos"));
        let loops_exists = muts_passed.contains(&String::from("Loops"));
        let reverse_exists = muts_passed.contains(&String::from("Reverse"));
        let shift_exists = muts_passed.contains(&String::from("Shift"));
        let shuffle_exists = muts_passed.contains(&String::from("Shuffle"));
        let swap_exists = muts_passed.contains(&String::from("Swap"));
        let increase_exists = muts_passed.contains(&String::from("Increase"));
        let gradient_exists = muts_passed.contains(&String::from("Gradient"));

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

        // Memory maps the temporary output file.
        self.data = Loader::init_file_mut(
            input,
            format!("{}temp.{}", self.outdir, self.extension).as_str()
        ).unwrap();

        self
    }

    /// Configures the mutation passed with the Bender's configuration.
    pub fn configure_mutation(&mut self, mutation: &mut Box<dyn Mutation>) -> &mut Self {
        mutation.configure(Box::new(&self.config));
        self
    }

    /// Performs the mutation.
    /// 
    /// Also adds the mutation to the log.
    pub fn mutate_with(&mut self, mutation: &mut Box<dyn Mutation>) -> &mut Self {
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
        let mut temp_muts = self.log.join("---");
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

        println!("{}temp.{}", self.outdir, self.extension);

        // Renames temporary file to actual output name
        Loader::rename_file(
            format!("{}temp.{}", self.outdir, self.extension).as_str(),
            genoutput.as_str()
        ).unwrap();
    }
}
