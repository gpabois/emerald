use std::ops::Deref;

use super::{arena::NodeIndex, Ast, Node};


pub struct NodeRef<'tree> {
    /// Index of the node
    pub index: NodeIndex,
    pub(super) ast: &'tree Ast,
    pub(super) content: &'tree Node
}

impl<'tree> NodeRef<'tree> {
    /// Downgrade the reference to an index, releasing the underlying borrow.
    pub fn downgrade(self) -> NodeIndex {
        self.index
    }

    pub fn iter_children(&'tree self) -> impl Iterator<Item=NodeRef<'tree>> {
        self.iter_children_by_ast(self.ast)
    }
}

impl<'tree> Deref for NodeRef<'tree> {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        self.content
    }
}