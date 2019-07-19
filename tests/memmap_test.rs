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