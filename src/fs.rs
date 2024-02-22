use std::io::Read;
use std::{collections::VecDeque, path::PathBuf};

use crate::{jewel::Jewel, path::Path};

pub struct Walk<'a> {
    jewel: &'a Jewel,
    queue: Vec<DirEntry>,
}

impl<'a> Walk<'a> {
    fn new(jewel: &'a Jewel, path: &Path) -> Option<Self> {
        Some(Self {
            jewel,
            queue: read_dir(jewel, path)?.collect(),
        })
    }
}

impl<'a> Iterator for Walk<'a> {
    type Item = DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.queue.pop()?;
        let meta = entry.metadata()?;

        if meta.is_dir() || meta.is_symlink() {
            self.queue.extend(read_dir(self.jewel, &entry.path)?);
        }

        Some(entry)
    }
}

/// Walk from the current directory to all its descendants
pub fn walk<'a>(jewel: &'a Jewel, path: &Path) -> Option<Walk<'a>> {
    Walk::new(jewel, path)
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

/// Metadata information about a file.
/// Similar to [https://doc.rust-lang.org/std/fs/struct.Metadata.html]
pub struct Metadata {
    is_symlink: bool,
    is_shard: bool,
    std_meta: std::fs::Metadata,
}

impl Metadata {
    pub fn is_symlink(&self) -> bool {
        self.is_symlink
    }

    pub fn is_shard(&self) -> bool {
        self.is_shard
    }

    pub fn is_file(&self) -> bool {
        self.std_meta.is_file()
    }

    pub fn is_dir(&self) -> bool {
        self.std_meta.is_dir()
    }
}

/// Entries returned by the ReadDir iterator.
/// Similar to [https://doc.rust-lang.org/std/fs/struct.DirEntry.html]
pub struct DirEntry {
    path: Path,
    std_dir_entry: std::fs::DirEntry,
}

impl DirEntry {
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the metadata for the file that this entry points at.
    /// Similar to [https://doc.rust-lang.org/std/fs/struct.DirEntry.html]
    pub fn metadata(&self) -> Option<Metadata> {
        let std_meta = self.std_dir_entry.metadata().ok()?;

        if std_meta.is_file() {
            // A shard
            if let Some(ext) = self.std_dir_entry.path().extension() {
                if ext == "md" {
                    return Some(Metadata {
                        std_meta,
                        is_shard: true,
                        is_symlink: false,
                    });
                }
            } else if Symlink::is(&self.std_dir_entry.path()) {
                return Some(Metadata {
                    std_meta,
                    is_symlink: true,
                    is_shard: false,
                });
            }
        }

        Some(Metadata {
            std_meta,
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
        Some(DirEntry {
            path,
            std_dir_entry,
        })
    }
}

/// Returns an iterator over the entries within a directory.
/// Similar to [https://doc.rust-lang.org/std/fs/fn.read_dir.html]
pub fn read_dir(jewel: &Jewel, path: &Path) -> Option<ReadDir> {
    let canon: PathBuf = canonicalize(jewel, path)?;

    let meta = std::fs::metadata(&canon).ok()?;

    if meta.is_dir() {
        let std_read_dir = std::fs::read_dir(canon.clone()).ok()?;
        return Some(ReadDir {
            path: path.clone(),
            std_read_dir,
        });
    }

    // Follow the symbolic link
    if Symlink::is(&canon) {
        let lnk = Symlink::load_from_canon(&canon)?;

        let std_read_dir = std::fs::read_dir(lnk.target).ok()?;

        return Some(ReadDir {
            path: path.clone(),
            std_read_dir,
        });
    }

    None
}

/// Open a file from the jewel
pub fn open(jewel: &Jewel, path: &Path) -> Option<File> {
    let canon = canonicalize(jewel, path)?;
    File::open(canon).ok()
}

/// Returns the canonical, absolute form of a path with all intermediate components normalized and symbolic links resolved.
pub fn canonicalize(jewel: &Jewel, path: &Path) -> Option<PathBuf> {
    let mut canon = jewel.root.clone();
    let mut parts = path.parts().collect::<VecDeque<_>>();

    while let Some(part) = parts.pop_front() {
        canon.push(part);

        let meta = std::fs::metadata(&canon).ok()?;

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

    Some(canon)
}

pub type File = std::fs::File;


