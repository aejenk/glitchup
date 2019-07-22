use super::loaders::Loader;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub trait MutConfig {
    fn to_hashmap(&self) -> HashMap<String, MutOptionVal>;
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum MutOptionVal {
    OString(String),
    OInt(isize),
    OBool(bool),
    OArray(Vec<MutOptionVal>),
    OMap(HashMap<String, MutOptionVal>)
}

pub struct TomlProcessor;

impl TomlProcessor {
    // pub fn parse<'a, T : Deserialize<'a>>(filename: String) -> Result<T, String>{
    pub fn parse_toml_as_options<T : DeserializeOwned>(filename: &str) -> Result<T, String>{
        let mut result = Loader::load_file_as_string(filename).unwrap();

        // Parse the toml file into a serializable struct
        let config : T = toml::from_str(result.as_mut_str()).unwrap();

        Ok(config)
    }
}

