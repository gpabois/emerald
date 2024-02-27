use markdown::{to_mdast, Constructs, ParseOptions};

pub use markdown::unist::Position;

use super::traits::NodeConverter;

pub type ArenaNodeIndex = generational_arena::Index;
type Arena = generational_arena::Arena<Node>;

/// An AST owned by an arena.
#[derive(Default)]
pub struct Ast {
    arena: Arena,
    root: Option<ArenaNodeIndex>,
}

impl super::traits::NodeConverter for Ast {
    type Node = Node;
    type NodeRef = ArenaNodeIndex;

    fn insert_node(&mut self, node: Self::Node) -> Self::NodeRef {
        self.arena.insert(node)
    }

    from_node_types! {}
}

impl Ast {
    /// Build the shard AST from string.
    ///
    /// ´´´´
    /// let content = "
    /// ---
    /// title: My shard
    /// date: 01/01/01
    /// ---
    /// # Heading
    /// This is a content [property:: value]
    /// ";
    ///
    /// Ast::from_str(&content)
    /// ´´´
    pub fn from_str(input: &str) -> Option<Self> {
        let constructs = Constructs {
            frontmatter: true,
            ..Constructs::gfm()
        };

        let options = ParseOptions {
            constructs,
            ..ParseOptions::default()
        };

        let tree = to_mdast(input, &options).ok()?;

        let mut ast = Self::default();
        ast.root = ast.convert(tree);

        Some(ast)
    }
}

/// A node owned by an arena.
pub struct Node {
    position: Option<Position>,
    children: Vec<ArenaNodeIndex>,
    attributes: super::NodeAttributes,
    r#type: super::NodeType,
}

impl super::traits::Node for Node {
    fn get_type(&self) -> super::NodeType {
        self.r#type
    }

    fn get_attributes(&self) -> &super::NodeAttributes {
        &self.attributes
    }

    fn get_position(&self) -> Option<&super::Position> {
        self.position.as_ref()
    }
}
