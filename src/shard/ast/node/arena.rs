use markdown::{to_mdast, Constructs, ParseOptions};

pub use markdown::unist::Position;

use crate::shard::ast::walker::RefWalker;

use super::{r#ref::NodeRef, traits::NodeConverter};

pub type NodeIndex = generational_arena::Index;
type Arena = generational_arena::Arena<Node>;

/// An AST owned by an arena.
#[derive(Default)]
pub struct Ast {
    arena: Arena,
    root: Option<NodeIndex>,
}

impl super::traits::NodeConverter for Ast {
    type Node = Node;
    type NodeRef = NodeIndex;

    fn insert_node(&mut self, node: Self::Node) -> Self::NodeRef {
        self.arena.insert(node)
    }

    from_node_types! {}
}

impl Ast {
    /// Build the shard AST from string.
    ///
    /// ```
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
    /// ```
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

    pub fn walk_ref(&self) -> RefWalker<'_> {
        RefWalker::new(self, self.root)
    }

    pub fn walk_ref_from(&self, index: NodeIndex) -> RefWalker<'_> {
        RefWalker::new(self, Some(index))
    }

    /// Copy the entire tree into a new tree.
    pub fn fork(&self, from: NodeIndex) -> Ast {
        let mut forked = Self::default();
        forked.root = self.fork_node(&mut forked, from);
        forked
    }

    /// Fork a sequence of nodes
    fn fork_nodes(
        &self,
        to: &mut Ast,
        nodes: impl Iterator<Item = NodeIndex>,
    ) -> impl Iterator<Item = NodeIndex> {
        let mut forked = Vec::<Option<NodeIndex>>::default();

        for node in nodes {
            forked.push(self.fork_node(to, node));
        }

        forked.into_iter().flatten()
    }

    /// Fork the node, and add it to the destination.
    fn fork_node(&self, to: &mut Ast, src: NodeIndex) -> Option<NodeIndex> {
        if let Some(node) = self.get(src) {
            let node = Node {
                position: node.position.clone(),
                children: self.fork_nodes(to, node.children.iter().cloned()).collect(),
                attributes: node.attributes.clone(),
                r#type: node.r#type,
            };

            return Some(to.insert_node(node));
        }

        None
    }

    pub fn get_root(&self) -> Option<NodeRef<'_>> {
        self.root.map(|r| self.get(r)).flatten()
    }

    pub fn get(&self, index: NodeIndex) -> Option<NodeRef<'_>> {
        self.arena.get(index).map(|content| NodeRef {
            index,
            ast: self,
            content,
        })
    }
}

/// A node owned by an arena.
pub struct Node {
    position: Option<Position>,
    pub children: Vec<NodeIndex>,
    pub(super) attributes: super::NodeAttributes,
    r#type: super::NodeType,
}

impl Node {
    pub fn iter_children_by_ast<'tree>(
        &'tree self,
        ast: &'tree Ast,
    ) -> impl Iterator<Item = NodeRef<'tree>> {
        self.children.iter().flat_map(|&child| ast.get(child))
    }
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
