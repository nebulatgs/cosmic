use std::{fs, io, path::Path};

pub fn new(name: &str, c: bool) -> io::Result<()> {
    let path = Path::new(name);
    fs::create_dir(path)?;
    super::init::init(path, c);
    Ok(())
}
