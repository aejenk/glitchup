use super::loaders::Loader;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

/// An enum to describe possible options that can be used to configure a Mutation.
#[allow(dead_code)]
#[derive(Debug)]
pub enum MutOptionVal {
    /// Represents a `String`
    OString(String),

    /// Represents an `isize`
    OInt(isize),

    /// Represents a `bool`
    OBool(bool),

    /// Represents a list of supported values.
    OArray(Vec<MutOptionVal>),

    /// Represents a `HashMap`.
    /// 
    /// Normally represents a whole `MutConfig`.
    OMap(HashMap<String, MutOptionVal>),

    /// Represents an empty value.
    /// 
    /// Used in the case some options are missing, in which case
    /// a Mutation can keep its default.
    ONone()
}

/// A trait to describe a struct that stores configuration options.
/// Used to configure mutations.
pub trait MutConfig {

    /// Converts a `MutConfig` into a hashmap to be used by 
    /// a `Mutation`. This conversion happens to facilitate 
    /// the passing of options.
    /// 
    /// The `String` should represent the option name, and 
    /// the `MutOptionVal` should represent the value.
    fn to_hashmap(&self) -> HashMap<String, MutOptionVal>;
}

/// Processes a TOML file into a `MutConfig`
pub struct TomlProcessor;

impl TomlProcessor {
    /// Parses a `TOML` file into a `MutConfig` struct.
    pub fn parse_toml_as_options<T : DeserializeOwned + MutConfig>(filename: &str) -> Result<T, String>{
        let mut result = Loader::load_file_as_string(filename).unwrap();

        // Parse the toml file into a serializable struct
        let config : T = toml::from_str(result.as_mut_str()).unwrap_or_else(
            |err| {panic!("Error occured while serialising options: {}", err);}
        );

        Ok(config)
    }
}

