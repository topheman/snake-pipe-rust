use std::path::PathBuf;

pub fn resolve_path(path: PathBuf) -> std::io::Result<PathBuf> {
    if path.is_absolute() {
        return Ok(path);
    }
    let current_dir = std::env::current_dir()?;
    std::fs::canonicalize(format!("{}/{}", current_dir.display(), path.display()))
}
