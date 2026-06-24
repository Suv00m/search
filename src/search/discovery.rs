use std::path::PathBuf;
use walkdir::WalkDir;

pub fn dispatch_file_path(path: &PathBuf) -> Box<dyn Iterator<Item = PathBuf>> {
    if path.is_file() {
        Box::new(std::iter::once(path.clone()))
    } else {
        Box::new(
            WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
                .map(|e| e.into_path()),
        )
    }
}
