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
    pub fn parse_toml_as_options<T : DeserializeOwned>(filename: String) -> Result<T, String>{
        let mut contents = String::new();

        let result = Loader::load_file_as_string(filename.clone(), &mut contents);

        if let Err(error) = result {
            panic!("Error while loading file from {} : {}", filename, error);
        }

        let config : T = toml::from_str(contents.as_str()).unwrap();

        Ok(config)
    }
}

