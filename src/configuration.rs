use glitchconsole::options::{TomlProcessor, MutConfig, MutOptionVal};
use serde::Deserialize;
use glitchup_derive::MutConfig;
use std::collections::HashMap;

use glitchconsole::loaders::Loader;

/* A helper class to represent the bender's configuration */

/// The main configuration of the bender.
/// 
/// Represents the entire TOML options file.
#[derive(Debug, Deserialize, MutConfig)]
#[allow(unused_attributes, non_snake_case)] // pops up a warning for custom attributes apparently.
pub struct Configuration {
    /// The name of the input file.
    /// Is required.
    #[ignore]
    pub inputfile : String,

    /// The name of the output file.
    /// If not passed, the name of the input file is used.
    #[ignore]
    pub outputfile : Option<String>,

    /// The number of times to repeat the program.
    /// If not passed, the value defaults to 1.
    #[ignore]
    pub times : Option<isize>,

    /// A range of iterations.
    /// Specifies how many times each mutation is applied.
    /// A global option to be set for all relevant mutations.
    pub iterations: Vec<isize>,

    /// A range of chunksizes.
    /// Specifies the size of each chunk of *bytes* to mutate.
    /// A global option to be set for all relevant mutations.
    pub chunksize: Vec<isize>,

    /// A list of mutations to be used. 
    #[ignore]
    pub mutations: Vec<Vec<String>>,

    // Mutation configurations.
    pub VoidConfig: Option<VoidConfig>,
    pub ChaosConfig: Option<ChaosConfig>,
    pub LoopConfig: Option<LoopConfig>,
    pub ReverseConfig: Option<ReverseConfig>,
    pub ShiftConfig: Option<ShiftConfig>,
    pub ShuffleConfig: Option<ShuffleConfig>,
    pub SwapConfig: Option<SwapConfig>,
    pub IncreaseConfig: Option<IncreaseConfig>,
    pub GradientConfig: Option<GradientConfig>
}

/* Derivative configs */

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

impl Configuration {

    /// Creates a new configuration, already setup.
    pub fn new(config_filename: &str) -> Result<Self, String> {
        let mut conf : Configuration = TomlProcessor::parse_toml_as_options(config_filename).unwrap();

        if !Loader::file_exists(conf.inputfile.as_str()) {
            return Err(format!("File '{}' does not exist!", conf.inputfile));
        }

        conf.setup_config();

        Ok(conf)
    }

    /// Sets up the configurations.
    /// 
    /// Is a huge function due to multiple repeated boilerplate code.
    /// In the future, there might be macros etc. to improve the style.
    /// 
    /// To add your own mutation, you would need to set up its configuration in this function.
    fn setup_config(&mut self) {
        let muts_passed = self.mutations.concat();

        self.iterations = verify_num_option(&self.iterations, "iterations", "globally");
        self.chunksize = verify_num_option(&self.chunksize, "chunksize", "globally");

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
        if !void_exists         {self.VoidConfig         = None};
        if !chaos_exists        {self.ChaosConfig        = None};
        if !loops_exists        {self.LoopConfig         = None};
        if !reverse_exists      {self.ReverseConfig      = None};
        if !shift_exists        {self.ShiftConfig        = None};
        if !shuffle_exists      {self.ShuffleConfig      = None};
        if !swap_exists         {self.SwapConfig         = None};
        if !increase_exists     {self.IncreaseConfig     = None};
        if !gradient_exists     {self.GradientConfig     = None};

        // VoidConfig setup
        if self.VoidConfig.is_none() && void_exists {
            self.VoidConfig = Some(VoidConfig {
                iterations: Some(self.iterations.clone()),
                chunksize: Some(self.chunksize.clone())
            });
        }
        else if let (Some(x), true) = (&mut self.VoidConfig, void_exists) {
            x.chunksize = if let Some(ch) = &x.chunksize
                {Some(verify_num_option(&ch, "chunksize", "under '[VoidConfig]'"))}
                else {Some(self.chunksize.clone())};
            
            x.iterations = if let Some(ch) = &x.iterations
                {Some(verify_num_option(&ch, "iterations", "under '[VoidConfig]'"))}
                else {Some(self.iterations.clone())};
        };

        // ChaosConfig setup
        if self.ChaosConfig.is_none() && chaos_exists {
            self.ChaosConfig = Some(ChaosConfig {
                iterations: Some(self.iterations.clone()),
                chunksize: Some(self.chunksize.clone())
            });
        }
        else if let (Some(x), true) = (&mut self.ChaosConfig, chaos_exists) {
            x.chunksize = if let Some(ch) = &x.chunksize
                {Some(verify_num_option(&ch, "chunksize", "under '[ChaosConfig]'"))}
                else {Some(self.chunksize.clone())};
            
            x.iterations = if let Some(ch) = &x.iterations
                {Some(verify_num_option(&ch, "iterations", "under '[ChaosConfig]'"))}
                else {Some(self.iterations.clone())};
        };

        // LoopConfig setup
        if self.LoopConfig.is_none() && loops_exists {
            let example = r#"
                [LoopConfig]
                loops = [1,2]
            "#;

            panic!("You have added a 'Loops' mutation, but haven't passed its options.
                    \nSpecifically, the 'loops' option needs to be passed under '[LoopConfig]'.
                    \nThe following is an example:
                    \n{}", example);
        }
        else if let (Some(x), true) = (&mut self.LoopConfig, loops_exists) {
            x.chunksize = if let Some(ch) = &x.chunksize
                {Some(verify_num_option(&ch, "chunksize", "under '[LoopConfig]'"))}
                else {Some(self.chunksize.clone())};
            
            x.iterations = if let Some(ch) = &x.iterations
                {Some(verify_num_option(&ch, "iterations", "under '[LoopConfig]'"))}
                else {Some(self.iterations.clone())};

            if x.loops.is_none() {
                panic!("You have added a 'Loops' mutation, but haven't passed the 'loops' option.
                        \nFor example: 'loops = [1,5]'");
            }
            else if let Some(l) = &x.loops {
                x.loops = Some(verify_num_option(&l, "loops", "under '[LoopConfig]'"));
            };
        };

        // ReverseConfig setup
        if self.ReverseConfig.is_none() && reverse_exists {
            self.ReverseConfig = Some(ReverseConfig {
                iterations: Some(self.iterations.clone()),
                chunksize: Some(self.chunksize.clone())
            });
        }
        else if let (Some(x), true) = (&mut self.ReverseConfig, reverse_exists) {
            x.chunksize = if let Some(ch) = &x.chunksize
                {Some(verify_num_option(&ch, "chunksize", "under '[ReverseConfig]'"))}
                else {Some(self.chunksize.clone())};
            
            x.iterations = if let Some(ch) = &x.iterations
                {Some(verify_num_option(&ch, "iterations", "under '[ReverseConfig]'"))}
                else {Some(self.iterations.clone())};
        };

        // ShiftConfig setup
        if self.ShiftConfig.is_none() && shift_exists {
            self.ShiftConfig = Some(ShiftConfig {
                iterations: Some(self.iterations.clone()),
                chunksize: Some(self.chunksize.clone())
            });
        }
        else if let (Some(x), true) = (&mut self.ShiftConfig, shift_exists) {
            x.chunksize = if let Some(ch) = &x.chunksize
                {Some(verify_num_option(&ch, "chunksize", "under '[ShiftConfig]'"))}
                else {Some(self.chunksize.clone())};
            
            x.iterations = if let Some(ch) = &x.iterations
                {Some(verify_num_option(&ch, "iterations", "under '[ShiftConfig]'"))}
                else {Some(self.iterations.clone())};
        };

        // ShuffleConfig setup
        if self.ShuffleConfig.is_none() && shuffle_exists {
            self.ShuffleConfig = Some(ShuffleConfig {
                iterations: Some(self.iterations.clone()),
                chunksize: Some(self.chunksize.clone())
            });
        }
        else if let (Some(x), true) = (&mut self.ShuffleConfig, shuffle_exists) {
            x.chunksize = if let Some(ch) = &x.chunksize
                {Some(verify_num_option(&ch, "chunksize", "under '[ShuffleConfig]'"))}
                else {Some(self.chunksize.clone())};
            
            x.iterations = if let Some(ch) = &x.iterations
                {Some(verify_num_option(&ch, "iterations", "under '[ShuffleConfig]'"))}
                else {Some(self.iterations.clone())};
        };

        // SwapConfig setup
        if self.SwapConfig.is_none() && swap_exists {
            self.SwapConfig = Some(SwapConfig {
                iterations: Some(self.iterations.clone()),
                chunksize: Some(self.chunksize.clone())
            });
        }
        else if let (Some(x), true) = (&mut self.SwapConfig, swap_exists) {
            x.chunksize = if let Some(ch) = &x.chunksize
                {Some(verify_num_option(&ch, "chunksize", "under '[SwapConfig]'"))}
                else {Some(self.chunksize.clone())};
            
            x.iterations = if let Some(ch) = &x.iterations
                {Some(verify_num_option(&ch, "iterations", "under '[SwapConfig]'"))}
                else {Some(self.iterations.clone())};
        };

        // IncreaseConfig setup
        if self.IncreaseConfig.is_none() && increase_exists {
            let example = r#"
                [IncreaseConfig]
                increase_by = [1,2]
            "#;

            panic!("You have added a 'IncreaseConfig' mutation, but haven't passed its options.
                    \nSpecifically, the 'increase_by' option needs to be passed under '[IncreaseConfig]'.
                    \nThe following is an example:
                    \n{}", example);
        }
        else if let (Some(x), true) = (&mut self.IncreaseConfig, increase_exists) {
            x.chunksize = if let Some(ch) = &x.chunksize
                {Some(verify_num_option(&ch, "chunksize", "under '[IncreaseConfig]'"))}
                else {Some(self.chunksize.clone())};
            
            x.iterations = if let Some(ch) = &x.iterations
                {Some(verify_num_option(&ch, "iterations", "under '[IncreaseConfig]'"))}
                else {Some(self.iterations.clone())};

            if x.increase_by.is_none() {
                panic!("You have added a 'Increase' mutation, but haven't passed the 'increase_by' option.
                        \nFor example: 'increase_by = [1,5]'");
            }
            else if let Some(l) = &x.increase_by {
                x.increase_by = Some(verify_num_option(&l, "increase_by", "under '[LoopConfig]'"));
            };
        };

        // GradientConfig setup
        if self.GradientConfig.is_none() && gradient_exists {
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
        else if let (Some(x), true) = (&mut self.GradientConfig, gradient_exists) {
            x.chunksize = if let Some(ch) = &x.chunksize
                {Some(verify_num_option(&ch, "chunksize", "under '[GradientConfig]'"))}
                else {Some(self.chunksize.clone())};
            
            x.iterations = if let Some(ch) = &x.iterations
                {Some(verify_num_option(&ch, "iterations", "under '[GradientConfig]'"))}
                else {Some(self.iterations.clone())};

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
}