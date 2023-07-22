use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    let mut assets = PathBuf::new();
    assets.push(env::var("CARGO_MANIFEST_DIR").unwrap());
    assets.pop();
    assets.push("target");
    assets.push(env::var("PROFILE").unwrap());
    assets.push("flight");
    assets.push("assets");
    if assets.exists() {
        fs::remove_dir_all(&assets).unwrap();
    }
    copy_dir_all("assets", assets).unwrap();
}