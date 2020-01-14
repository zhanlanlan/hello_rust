use std::path::Path;
use std::path::PathBuf;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct ListDirResult(pub Vec<PathBuf>);

impl ListDirResult {
    pub fn listdir(&mut self, path: &PathBuf) {
        for entry in path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    self.listdir(&path);
                }
                self.0.push(path);
            }
        }
    }
}

pub fn run() {
    let now = Instant::now();
    let path = PathBuf::from("/Users/liuyuan");
    let mut result = ListDirResult(Vec::<PathBuf>::with_capacity(2000000));
    println!("len: {}", result.0.capacity());
    result.listdir(&path);
    eprintln!("result: {}", result.0.len());
    eprint!("cost time: {} ms", now.elapsed().as_millis());
}
