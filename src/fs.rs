use std::error::Error;
use std::io::Read;
use std::{collections::VecDeque, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{path::Path, Emerald};

pub struct Walk {
    jewel: Emerald,
    queue: Vec<DirEntry>,
}

impl Walk {
    fn new(jewel: Emerald, path: &Path) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            jewel: jewel.clone(),
            queue: read_dir(&jewel, path)?.collect(),
        })
    }
}

impl Iterator for Walk {
    type Item = DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.queue.pop()?;
        let meta = entry.metadata();

        if meta.is_dir() || meta.is_symlink() {
            self.queue
                .extend(read_dir(&self.jewel, &entry.path).unwrap());
        }

        Some(entry)
    }
}

/// Walk from the current directory to all its descendants
pub fn walk(jewel: &Emerald, path: &Path) -> Result<Walk, Box<dyn Error>> {
    Walk::new(jewel.clone(), path)
}

/// A symlink information
pub struct Symlink {
    pub name: String,
    target: PathBuf,
}

impl Symlink {
    fn is(path: &std::path::Path) -> bool {
        Self::has_magic(path).unwrap_or(false)
    }

    fn has_magic(path: &std::path::Path) -> Option<bool> {
        let mut buf = <[u8; 3]>::default();
        let mut file = std::fs::File::open(path).ok()?;
        file.read_exact(&mut buf).ok();
        let magic = std::str::from_utf8(&buf).ok()?;
        Some(magic == "@/>")
    }

    /// Read the symbolic link file from the canonical path.
    fn load_from_canon(path: &std::path::Path) -> Option<Self> {
        let name = path.file_name()?.to_str()?.to_string();
        let stream = std::fs::read_to_string(path).ok()?;

        if stream.starts_with("@/>") {
            let (_, target) = stream.split_once("@/>")?;

            return Some(Symlink {
                name,
                target: std::path::Path::new(target).into(),
            });
        }

        None
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
/// Metadata information about a file.
/// Similar to [https://doc.rust-lang.org/std/fs/struct.Metadata.html]
pub struct Metadata {
    is_symlink: bool,
    is_shard: bool,
    is_file: bool,
    is_dir: bool,
}

impl Metadata {
    pub fn is_symlink(&self) -> bool {
        self.is_symlink
    }

    pub fn is_shard(&self) -> bool {
        self.is_shard
    }

    pub fn is_file(&self) -> bool {
        self.is_file
    }

    pub fn is_dir(&self) -> bool {
        self.is_dir
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
/// Entries returned by the ReadDir iterator.
/// Similar to [https://doc.rust-lang.org/std/fs/struct.DirEntry.html]
pub struct DirEntry {
    path: Path,
    metadata: Metadata,
}

impl DirEntry {
    fn new(path: Path, metadata: Metadata) -> Self {
        Self { path, metadata }
    }

    fn from_std_dir_entry(
        path: &Path,
        dir_entry: std::fs::DirEntry,
    ) -> Result<Self, Box<dyn Error>> {
        let meta = Self::read_metadata(dir_entry)?;
        Ok(Self::new(path.clone(), meta))
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the metadata for the file that this entry points at.
    /// Similar to [https://doc.rust-lang.org/std/fs/struct.DirEntry.html]
    fn read_metadata(dir_entry: std::fs::DirEntry) -> Result<Metadata, Box<dyn Error>> {
        let std_meta = dir_entry.metadata()?;

        if std_meta.is_file() {
            // A shard
            if let Some(ext) = dir_entry.path().extension() {
                if ext == "md" {
                    return Ok(Metadata {
                        is_file: true,
                        is_dir: false,
                        is_shard: true,
                        is_symlink: false,
                    });
                }
            } else if Symlink::is(&dir_entry.path()) {
                return Ok(Metadata {
                    is_dir: false,
                    is_file: false,
                    is_symlink: true,
                    is_shard: false,
                });
            }
        }

        Ok(Metadata {
            is_dir: std_meta.is_dir(),
            is_file: std_meta.is_file(),
            is_shard: false,
            is_symlink: false,
        })
    }
}

/// Iterator over the entries in a directory.
/// Similar to [https://doc.rust-lang.org/std/fs/struct.ReadDir.html]
pub struct ReadDir {
    path: Path,
    std_read_dir: std::fs::ReadDir,
}

impl Iterator for ReadDir {
    type Item = DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let std_dir_entry = self.std_read_dir.next()?.ok()?;
        let mut path = self.path.clone();
        path.append(std_dir_entry.file_name().to_str()?);

        let entry = DirEntry::from_std_dir_entry(&path, std_dir_entry).unwrap();
        Some(entry)
    }
}

/// Returns an iterator over the entries within a directory.
/// Similar to [https://doc.rust-lang.org/std/fs/fn.read_dir.html]
pub fn read_dir(jewel: &Emerald, path: &Path) -> Result<ReadDir, Box<dyn Error>> {
    let canon: PathBuf = canonicalize(jewel, path)?;

    let meta = std::fs::metadata(&canon)?;

    if meta.is_dir() {
        let std_read_dir = std::fs::read_dir(canon.clone())?;
        return Ok(ReadDir {
            path: path.clone(),
            std_read_dir,
        });
    }

    // Follow the symbolic link
    if Symlink::is(&canon) {
        let lnk = Symlink::load_from_canon(&canon).unwrap();

        let std_read_dir = std::fs::read_dir(lnk.target)?;

        return Ok(ReadDir {
            path: path.clone(),
            std_read_dir,
        });
    }

    panic!("not a directory or a symlink")
}

/// Open a file from the Emerald
pub fn open(emerald: &Emerald, path: &Path) -> Result<File, Box<dyn Error>> {
    File::open(emerald, path)
}

/// Returns the canonical, absolute form of a path with all intermediate components normalized and symbolic links resolved.
pub fn canonicalize(jewel: &Emerald, path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let mut canon = jewel.get_root().to_owned();
    let mut parts = path.parts().collect::<VecDeque<_>>();

    while let Some(part) = parts.pop_front() {
        canon.push(part);

        let meta = std::fs::metadata(&canon)?;

        // We reached a file, but it is not the leaf
        // Either we have a symlink, or it is an invalid path
        if parts.is_empty() && meta.is_file() {
            // Check if we have a symlink,
            // and only if the symlink is not the leaf.
            // Replace the current canon
            if let Some(lnk) = Symlink::load_from_canon(&canon) {
                canon = lnk.target;
            }
        }
    }

    Ok(canon)
}

pub struct File(std::fs::File);

impl File {
    pub fn open(emerald: &Emerald, path: &Path) -> Result<Self, Box<dyn Error>> {
        let canon = canonicalize(emerald, path)?;
        Ok(Self(std::fs::File::open(canon)?))
    }
}
