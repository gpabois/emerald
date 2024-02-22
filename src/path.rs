#[derive(Default, Clone)]
/// A path in the jewel fs
pub struct Path(String);

impl Path {
    pub fn new(value: &str) -> Option<Self> {
        Some(Self(value.into()))
    }
}

pub type Part<'a> = &'a str;

impl std::fmt::Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl std::string::ToString for Path {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl Path {
    pub fn append(&mut self, part: &str) {
        if !part.starts_with('/') {
            self.0.push('/');
        }
        self.0.push_str(part);
    }

    pub fn parts(&self) -> impl Iterator<Item = Part<'_>> {
        self.0.split('/')
    }
}

