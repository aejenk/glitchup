use std::fs::{OpenOptions, copy};
use std::io::Read;
use memmap::MmapMut;
use std::path::PathBuf;

/// A Loader struct to facilitate file manipulation (loading, memorymapping...)
pub struct Loader;

impl Loader {
    /// Copies a file from `from` to `to`. 
    pub fn copy_file(from: &str, to: &str) -> std::io::Result<()>{
        copy(from, to)?;
        Ok(())
    }

    /// Constructs a mutable memory map of file at `name`.
    pub fn map_file_mut(name: &str) -> std::io::Result<memmap::MmapMut> {
        let path : PathBuf = PathBuf::from(name);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)?;

        let mmap = unsafe { MmapMut::map_mut(&file)? };

        Ok(mmap)
    }

    /// A combination of `copy_file` and `map_file_mut`.
    pub fn init_file_mut(from: &str, to:&str) -> std::io::Result<memmap::MmapMut> {
        Loader::copy_file(from, to.clone())?;
        Loader::map_file_mut(to)
    }

    /// Loads the contents of the file at `name` and returns it as a `String`.
    pub fn load_file_as_string(name: &str) -> std::io::Result<String> {
        let mut output = String::new();
        let mut file = OpenOptions::new()
            .read(true)
            .open(&name)?;

        file.read_to_string(&mut output)?;

        Ok(output)
    }
}