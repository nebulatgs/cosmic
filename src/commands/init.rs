use std::{io, path::Path};

pub fn init(path: &Path, c: bool) -> io::Result<()> {
    let src = path.join("src");
    std::fs::create_dir(&path)?;
    let file = if c {
        src.join("main.c")
    } else {
        src.join("main.cpp")
    };
    std::fs::write(&file, "")?;
    Ok(())
}
