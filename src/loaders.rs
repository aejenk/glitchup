use std::fs::{OpenOptions, copy};
use std::io::Read;
use memmap::MmapMut;
use std::path::PathBuf;

pub struct Loader;

impl Loader {
    pub fn copy_file(from: String, to: String) -> std::io::Result<()>{
        copy(from, to)?;
        Ok(())
    }

    pub fn map_file_mut(name: String) -> std::io::Result<memmap::MmapMut> {
        let path : PathBuf = PathBuf::from(name);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)?;

        let mmap = unsafe { MmapMut::map_mut(&file)? };

        Ok(mmap)
    }

    pub fn init_file_mut(from: String, to:String) -> std::io::Result<memmap::MmapMut> {
        Loader::copy_file(from, to.clone())?;
        Loader::map_file_mut(to)
    }

    pub fn load_file_as_string(name: String, output: &mut String) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(&name)?;

        file.read_to_string(output)?;

        Ok(())
    }
}