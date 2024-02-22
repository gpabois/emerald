use crate::fs::{File, Walk};
use crate::path::Path;
use crate::shard::Shard;

use std::ffi::OsStr;

pub struct Jewel {
    pub(crate) root: std::path::PathBuf
}

impl Jewel {
    /// Open a Jewel
    pub fn open<S>(root: &S) -> Option<Self> where S: AsRef<OsStr> + ?Sized {
        let root = std::path::Path::new(root);
        
        if !std::fs::metadata(root).ok()?.is_dir() {
            return  None;
        }

        Some(Jewel {
            root: root.into()
        })
    }

    /// Walk over all files within the Jewel
    pub fn walk(&self) -> Walk<'_> {
        crate::fs::walk(&self, &crate::path::Path::default()).unwrap()
    }

    pub fn walk_shards(&self) -> impl Iterator<Item=Shard> + '_ {
        self.walk()
        .filter(|entry|  entry.metadata().unwrap().is_shard())
        .map(|entry| {
            let path = entry.path();
            self.open_file(path)
        })
        .filter(|maybe_file| maybe_file.is_some())
        .map(|maybe_file| maybe_file.unwrap())
        .map(|mut file| Shard::read(file))
        .filter(|maybe_shard| maybe_shard.is_some())
        .map(|maybe_shard| maybe_shard.unwrap())
    }

    /// Open a file from the Jewel
    pub fn open_file(&self, path: &Path) -> Option<File> {
        crate::fs::open(self, path)
    }
}
