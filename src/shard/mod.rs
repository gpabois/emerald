use markdown::mdast::Node;

use self::header::Header;

mod ast;

pub mod header;

#[derive(Debug)]
pub struct Shard {
    header: Option<Header>,
    root: Node
}

impl Shard {
    pub fn from_str(input: &str) -> Option<Self> {
        let root = ast::parse_ast(input)?;
        let header = Header::from_frontmatter(ast::find_frontmatter(&root));
        Some(Self{header, root})
    }

    pub fn iter_raw_variables(&self, max_depth: isize) -> impl Iterator<Item=RawVariable<'_>> {
        ast::iter_raw_variables(&self.root, max_depth)
    }
}