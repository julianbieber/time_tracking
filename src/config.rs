
use std::path::{PathBuf, Path};
use dirs::home_dir;

pub fn base_path() -> PathBuf {
    home_dir()
        .expect("Home directory must be set")
        .join(Path::new(".tracking"))
}