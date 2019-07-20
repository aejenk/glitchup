extern crate benders;

#[test]
// A temporary test to keep track of how to use memmap.
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
fn basic_mut_test() -> std::io::Result<()>{
    use memmap::MmapMut;
    use std::path::PathBuf;
    use std::fs::OpenOptions;
    use std::io::{Write,Read};
    use std::collections::HashMap;
    use benders::*;
    use MutOptionVal::*;

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

    let mut basic_mut = BasicMutation::default();
    let mut options = HashMap::new();

    options.insert(String::from("chunk_size"), OInt(3));
    options.insert(String::from("min"), OInt(1));
    options.insert(String::from("max"), OInt(2));

    basic_mut.mutate(&mut *mmap, options);

    // Reload and read content of file
    let mut file = OpenOptions::new()
        .read(true)
        .open(&path)?;

    let mut chars = String::new();
    file.read_to_string(&mut chars)?;

    // Check characters were overwritten
    assert_eq!(chars.as_bytes(), b"[000] Hello world!");

    Ok(())
}