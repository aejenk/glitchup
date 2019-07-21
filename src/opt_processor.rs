use super::loaders::Loader;
use toml::Value as Toml;
use serde::de::DeserializeOwned;

pub struct OptionProcessor;



impl OptionProcessor {
    // pub fn parse<'a, T : Deserialize<'a>>(filename: String) -> Result<T, String>{
    pub fn parse<T : DeserializeOwned>(filename: String) -> Result<T, String>{
        let mut contents = String::new();

        let result = Loader::load_file_as_string(filename.clone(), &mut contents);

        if let Err(error) = result {
            panic!("Error while loading file from {} : {}", filename, error);
        }

        let config : T = toml::from_str(contents.as_str()).unwrap();

        Ok(config)
    }
}

