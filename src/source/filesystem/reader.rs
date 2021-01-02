use anyhow::Result;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

pub struct AdditionReader {
    sizes: HashMap<PathBuf, u64>,
}

impl AdditionReader {
    pub fn new(files: Vec<PathBuf>) -> Result<Self> {
        let mut sizes = HashMap::new();

        for path in files {
            scan_sizes(&mut sizes, path)?;
        }

        Ok(AdditionReader { sizes })
    }
    pub fn scan(&mut self, path: PathBuf) -> Result<()> {
        scan_sizes(&mut self.sizes, path)
    }
    pub fn read_addition(&mut self, path: PathBuf) -> Result<String> {
        let old_len = self.sizes.get(&path).cloned().unwrap_or(0);
        let current_len = path.metadata()?.len();
        let seek_to = if current_len > old_len { old_len } else { 0 };

        let mut buffer = Vec::new();
        let mut file = File::open(&path)?;

        file.seek(SeekFrom::Start(seek_to))?;
        file.read_to_end(&mut buffer)?;

        let addition = String::from_utf8_lossy(&buffer).to_string();
        self.sizes.insert(path, current_len);

        Ok(addition)
    }
}

fn scan_sizes(map: &mut HashMap<PathBuf, u64>, path: PathBuf) -> Result<()> {
    let meta = path.metadata()?;
    if meta.is_dir() {
        for entry in fs::read_dir(path)? {
            scan_sizes(map, entry?.path())?;
        }
    } else {
        map.insert(path, meta.len());
    }

    Ok(())
}
