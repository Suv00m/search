use std::fs;
use std::iter::empty;
use std::path::Path;
use std::path::PathBuf;

pub fn dispatch_file_path(path: &Path) -> Box<dyn Iterator<Item = PathBuf>> {
    if path.exists() {
        if path.is_dir() {
            return Box::new(fs::read_dir(path).unwrap().map(|x| x.unwrap().path()));
        } else {
            return Box::new(vec![path.to_path_buf()].into_iter());
        }
    } else {
        return Box::new(empty());
    }
}
