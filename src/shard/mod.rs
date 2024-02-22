mod ast;

pub mod task;
pub mod header;

use markdown::mdast::Node;
use crate::path::Path;
use header::Header;
use task::Task;


#[derive(Debug)]
/// A shard is a piece of data within a Jewel.
pub struct Shard {
    /// Location of the shard
    pub path: Path,
    /// Properties of the shard
    properties: Option<Header>,
    /// AST
    pub ast: Node
}

impl Shard {
    /// Read the shard from a stream.
    pub fn read<R: std::io::Read>(mut stream: R) -> Option<Self> {
        let mut doc = String::default();
        stream.read_to_string(&mut doc);
        Self::from_str(&doc)
    }

    /// Read the shard from a string.
    pub fn from_str(input: &str) -> Option<Self> {
        let ast = ast::parse_ast(input)?;
        let properties = Header::from_frontmatter(ast::find_frontmatter(&ast));
        Some(Self{properties, ast})
    }

    /// Iterate over all the tasks within the shard.
    pub fn iter_tasks(&self) -> impl Iterator<Item=Task> + '_ {
        task::iter(&self.path, &self.ast)
    }
}
