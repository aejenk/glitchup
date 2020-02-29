use std::fs;
use cfgmap::{CfgMap, Checkable, Condition::*};
use std::ops::Deref;

/* A helper class to represent the bender's configuration */

/// The main configuration of the bender.
/// 
/// Represents the entire TOML options file.
pub struct Configuration {
    cfg: CfgMap
}

impl Configuration {

    pub fn from_file(config_filename: &str) -> Result<Self, String> {
        let file = fs::read_to_string(config_filename).expect("Failed to read file into string.");
        Ok(Configuration { cfg: toml::from_str::<toml::Value>(&file).expect("Couldn't parse as toml.").into() })
    }

    /// REDO DOC
    pub fn verify_config(&mut self) {
        let muts_passed : Vec<&String> = self.get_mutations().into_iter().flatten().collect();

        static POSSIBLE_MUTS : [&str; 11]= ["Void", "Chaos", "Loops", "Reverse", "Shift", "Shuffle", "Swap",
                                           "Increase", "Gradient", "Multiply", "Compress"];

        for string in &muts_passed {
            if !POSSIBLE_MUTS.contains(&string.as_str()) {
                panic!("Invalid mutation: {:?}\n\tOnly allowed mutations: {:#?}", string, POSSIBLE_MUTS);
            }
        }
    }

    pub fn get_mutations(&self) -> Vec<Vec<&String>> {
        self.get("mutations").unwrap().as_list().unwrap()
            .into_iter()
            .map(|mutation| 
                mutation.as_list().unwrap().into_iter().map(
                    |s| s.as_str().unwrap()
                ).collect())
            .collect()
    } 

    pub fn get_option_as_range_int(&self, category: &str, value: &str) -> Option<(i64, i64)> {
        let valid = self.get_option(category, value)
            .check_that(IsListWith(Box::new(IsInt)) & IsListWithLength(2));

        if valid {
            let start = format!("{}/0", value);
            let end = format!("{}/1", value);

            Some((
                *self.get_option(category, &start).unwrap().as_int().unwrap(),
                *self.get_option(category, &end).unwrap().as_int().unwrap(),
            ))
        }
        else {
            None
        }
    }

    pub fn get_option_as_single_int(&self, category: &str, value: &str) -> Option<i64> {
        self.get_option(category, value)
            .and_then(|v| v.as_int())
            .map(|i| *i)
    }

    pub fn get_option_as_range_float(&self, category: &str, value: &str) -> Option<(f64, f64)> {
        let valid = self.get_option(category, value)
            .check_that(IsListWith(Box::new(IsFloat)) & IsListWithLength(2));

        if valid {
            let start = format!("{}/0", value);
            let end = format!("{}/1", value);

            Some((
                *self.get_option(category, &start).unwrap().as_float().unwrap(),
                *self.get_option(category, &end).unwrap().as_float().unwrap(),
            ))
        }
        else {
            None
        }
    }

    pub fn get_option_as_single_float(&self, category: &str, value: &str) -> Option<f64> {
        self.get_option(category, value)
            .and_then(|v| v.as_float())
            .map(|i| *i)
    }

    pub fn get_inputfile(&self) -> &str {
        self.get("inputfile")
            .expect("Must specify 'inputfile' option globally.")
            .as_str()
            .expect("Must specify 'inputfile' as a string.")
    }
}

impl Deref for Configuration {
    type Target = CfgMap;

    fn deref(&self) -> &Self::Target {
        &self.cfg
    }
}