use std::{error::Error, ffi::OsStr};

pub mod emerald;
pub mod fs;
pub mod path;

pub mod script;
pub mod shard;
pub use emerald::Emerald;

/// Open the jewel
pub fn open<S>(root: &S) -> Result<Emerald, Box<dyn Error>>
where
    S: AsRef<OsStr> + ?Sized,
{
    Emerald::open(root)
}
