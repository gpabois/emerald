use std::{error::Error, ffi::OsStr, sync::Arc};

struct Inner {
    pub(crate) root: std::path::PathBuf,
}

#[derive(Clone)]
/// Main object
pub struct Emerald(Arc<Inner>);

impl Emerald {
    /// Open an emerald
    pub fn open<S>(root: &S) -> Result<Self, Box<dyn Error>>
    where
        S: AsRef<OsStr> + ?Sized,
    {
        let root = std::path::Path::new(root);

        if !std::fs::metadata(root)?.is_dir() {
            panic!("not a directory")
        }

        Ok(Emerald(Arc::new(Inner { root: root.into() })))
    }

    pub fn get_root(&self) -> &std::path::Path {
        &self.0.root
    }
}
