use std::fs::OpenOptions;
use std::path::PathBuf;
use memmap::MmapMut;

pub fn memmap_test() -> std::io::Result<()>{
    let path : PathBuf = PathBuf::from("foo.txt");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)?;
    let mut mmap = unsafe { MmapMut::map_mut(&file)? };

    if let Some(slice) = mmap.get_mut(0..3) {
        for chr in slice.iter_mut() {
            *chr+=1;
        }
    }
    Ok(())
}