use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
/// A path in the jewel fs
pub struct Path {
    inner: String,
}

impl Path {
    pub fn new(value: &str) -> Option<Self> {
        Some(Self {
            inner: value.into(),
        })
    }
}

pub type Part<'a> = &'a str;

impl std::fmt::Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.inner)
    }
}

impl std::string::ToString for Path {
    fn to_string(&self) -> String {
        self.inner.clone()
    }
}

impl Path {
    pub fn append(&mut self, part: &str) {
        if !part.starts_with('/') {
            self.inner.push('/');
        }
        self.inner.push_str(part);
    }

    pub fn parts(&self) -> impl Iterator<Item = Part<'_>> {
        self.inner.split('/')
    }
}
