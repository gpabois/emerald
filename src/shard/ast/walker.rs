use super::{arena::NodeIndex, r#ref::NodeRef, Ast};
use std::collections::VecDeque;

pub struct Cursor {
    depth: isize,
    index: NodeIndex,
}

pub enum WalkerMode {
    /// Depth-first walking
    Depth,
    /// Breadth-first walking
    Breadth,
}

impl Default for WalkerMode {
    fn default() -> Self {
        Self::Breadth
    }
}

/// Recursively iterate over all nodes in the AST
pub struct RefWalker<'tree> {
    ast: &'tree Ast,
    queue: VecDeque<Cursor>,
    mode: WalkerMode,
    max_depth: isize,
}

impl<'tree> RefWalker<'tree> {
    pub(super) fn new(ast: &'tree Ast, node: Option<NodeIndex>) -> Self {
        Self {
            ast,
            queue: node
                .into_iter()
                .map(|node| Cursor {
                    depth: 0,
                    index: node,
                })
                .collect(),
            max_depth: -1,
            mode: WalkerMode::default(),
        }
    }

    pub fn mode(mut self, mode: WalkerMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn max_depth(mut self, depth: isize) -> Self {
        self.max_depth = depth;
        self
    }

    fn pop(&mut self) -> Option<Cursor> {
        match self.mode {
            WalkerMode::Depth => self.queue.pop_back(),
            WalkerMode::Breadth => self.queue.pop_front(),
        }
    }

    fn has_reached_depth_limit(&self, cursor: &Cursor) -> bool {
        self.max_depth > 0 && self.max_depth < cursor.depth
    }

    fn include_children(&self, cursor: &Cursor) -> bool {
        !self.has_reached_depth_limit(cursor)
    }
}

impl<'tree> Iterator for RefWalker<'tree> {
    type Item = NodeRef<'tree>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(cursor) = self.pop() {
            if let Some(node) = self.ast.get(cursor.index) {
                if self.include_children(&cursor) {
                    self.queue.extend(node.children.iter().map(|&index| Cursor {
                        depth: cursor.depth + 1,
                        index,
                    }));
                }

                return Some(node);
            }
        }

        None
    }
}

