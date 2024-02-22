use std::ffi::OsStr;

pub mod fs;
pub mod path;

pub mod jewel;
pub mod shard;

pub use jewel::Jewel;

/// Open the jewel
pub fn open<S>(root: &S) -> Option<Jewel>
where
    S: AsRef<OsStr> + ?Sized,
{
    Jewel::open(root)
}
