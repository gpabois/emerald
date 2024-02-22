use crate::fs::{self, File, Walk};
use crate::path::Path;
use crate::shard::Shard;

use std::ffi::OsStr;

pub struct Jewel {
    pub(crate) root: std::path::PathBuf,
}

impl Jewel {
    /// Open a Jewel
    pub fn open<S>(root: &S) -> Option<Self>
    where
        S: AsRef<OsStr> + ?Sized,
    {
        let root = std::path::Path::new(root);

        if !std::fs::metadata(root).ok()?.is_dir() {
            return None;
        }

        Some(Jewel { root: root.into() })
    }

    /// Walk over all files within the Jewel
    pub fn walk(&self) -> Walk<'_> {
        fs::walk(self, &Path::default()).unwrap()
    }

    pub fn walk_shards(&self) -> impl Iterator<Item = Shard> + '_ {
        self.walk()
            .filter(|entry| entry.metadata().unwrap().is_shard())
            .map(|entry| {
                let path = entry.path();
                (path.clone(), self.open_file(path))
            })
            .filter(|(_, maybe_file)| maybe_file.is_some())
            .map(|(path, maybe_file)| (path, maybe_file.unwrap()))
            .flat_map(|(path, file)| Shard::read(&path, file))
    }

    /// Open a file from the Jewel
    pub fn open_file(&self, path: &Path) -> Option<File> {
        fs::open(self, path)
    }
}
