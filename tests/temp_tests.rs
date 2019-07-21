extern crate benders;

#[test]
// A temporary test to keep track of how to use memmap.
// Also serves as an initialiser
fn memmap_test() -> std::io::Result<()>{
    use memmap::MmapMut;
    use std::path::PathBuf;
    use std::fs::OpenOptions;
    use std::io::{Write, Read};

    // Create a file called "foo.txt"
    let path : PathBuf = PathBuf::from("foo.txt");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)?;

    // Writes bytes to file
    file.write_all(b"[!!!] Hello world!")?;

    // Mutably memory map it
    let mut mmap = unsafe { MmapMut::map_mut(&file)? };

    // Set some characters to x
    if let Some(slice) = mmap.get_mut(1..4) {
        for chr in slice.iter_mut() {
            *chr = b'x';
        }
    }

    // Reload and read content of file
    let mut file = OpenOptions::new()
        .read(true)
        .open(&path)?;

    let mut chars = String::new();
    file.read_to_string(&mut chars)?;

    // Check characters were overwritten
    assert_eq!(chars.as_bytes(), b"[xxx] Hello world!");

    Ok(())
}

#[test]
// A temporary test to show functionality of basic mutation.
// Also shows the use of Loaders
fn basic_mut_test() -> std::io::Result<()>{
    // libs needed for benders
    use std::collections::HashMap;  
    use benders::mutations::*;      // import mutations
    use MutOptionVal::*;            // import enum from core
    use benders::loaders::Loader;

    // file name for testing
    let fname : String = String::from("tests/foo.txt");

    // Mutably memory map it
    let mut mmap = Loader::map_file_mut(fname.clone())?;

    // Initialise mutation and options
    let mut basic_mut = BasicMutation::default();
    let mut options = HashMap::new();

    // Add options to map
    options.insert(String::from("chunk_size"), OInt(3));
    options.insert(String::from("min"), OInt(1));
    options.insert(String::from("max"), OInt(2));

    // Mutate file
    basic_mut.mutate(&mut *mmap, options);

    // Reload and read content of file
    let mut chars = String::new();
    Loader::load_file_as_string(fname, &mut chars)?;

    // Check characters were overwritten
    assert_eq!(chars.as_bytes(), b"[000] Hello world!");

    Ok(())
}