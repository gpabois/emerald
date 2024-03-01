use std::fmt::Debug;

use super::{traits::Node, Ast, NodeRef};

impl Debug for NodeRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeRef")
            .field("index", &self.index)
            .field("type", &self.content.get_type())
            .field("attributes", &self.content.get_attributes())
            .field("children", &self.iter_children().collect::<Vec<_>>())
            .finish()
    }
}

impl Debug for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ast")
            .field("root", &self.get_root())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::shard::ast::Ast;

    #[test]
    pub fn debug_ast() {
        let content = r#"---
title: My shard
date: 01/01/01
---

# Heading
This is a content [property:: value]
"#;

        let ast = Ast::from_str(content).unwrap();
        println!("{:#?}", ast);
    }
}
