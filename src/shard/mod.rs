mod ast;

pub mod header;
pub mod task;

use crate::path::Path;
use ast::Ast;
use header::Header;
use task::Task;

/// A shard is a piece of data within a Jewel.
pub struct Shard {
    /// Location of the shard
    pub path: Path,
    /// Properties of the shard
    pub properties: Option<Header>,
    /// AST
    pub ast: Ast,
}

impl Shard {
    /// Read the shard from a stream.
    pub fn read<R: std::io::Read>(path: &Path, mut stream: R) -> Option<Self> {
        let mut doc = String::default();
        stream.read_to_string(&mut doc).ok()?;
        Self::from_str(path, &doc)
    }

    /// Read the shard from a string.
    pub fn from_str(path: &Path, input: &str) -> Option<Self> {
        let ast = ast::Ast::from_str(input)?;
        let properties = Header::from_frontmatter(ast::find_frontmatter(&ast));
        Some(Self {
            path: path.to_owned(),
            properties,
            ast,
        })
    }

    /// Iterate over all the tasks within the shard.
    pub fn iter_tasks(&self) -> impl Iterator<Item = Task> + '_ {
        task::iter(&self.path, &self.ast)
    }
}
