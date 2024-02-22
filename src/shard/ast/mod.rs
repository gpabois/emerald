use markdown::{to_mdast, Constructs, ParseOptions};

pub use markdown::mdast::*;

pub fn parse_ast(input: &str) -> Option<Node> {
    let constructs = Constructs {
        frontmatter: true,
        ..Constructs::gfm()
    };

    let options = ParseOptions {
        constructs,
        ..ParseOptions::default()
    };

    to_mdast(input, &options).ok()
}

/// Walk all nodes
/// Don't include children if the predicate is true
pub struct StopWalkAst<'a, Predicate> 
where Predicate: Fn(&Node) -> bool
{
    queue: Vec<(&'a Node, isize)>,
    predicate: Predicate 
}

impl<'a, Predicate> Iterator for StopWalkAst<'a, Predicate> 
where Predicate: Fn(&Node) -> bool
{
    type Item = (&'a Node, isize); // (node, depth)

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((node, depth)) = self.queue.pop() {
            if !(self.predicate)(node) {
                if let Some(children) = node.children() {
                    self.queue.extend(children.iter().map(|node| (node, depth + 1)));
                }
            }

            return Some((node, depth))
        }
        None
    }
}

impl<'a, Predicate> StopWalkAst<'a, Predicate> 
where Predicate: Fn(&Node) -> bool
{
    pub fn limit_depth(self, max_depth: isize) -> impl Iterator<Item=&'a Node> {
        self.filter(move |(_, depth)| *depth <= max_depth).map(|(node, _)| node)
    }
}

/// Walk all the AST's nodes
pub struct WalkAst<'a> {
    queue: Vec<(&'a Node, isize)>
}

impl<'a> WalkAst<'a> {
    pub fn limit_depth(self, max_depth: isize) -> impl Iterator<Item=&'a Node> {
        self.filter(move |(_, depth)| *depth <= max_depth).map(|(node, _)| node)
    }
}

impl<'a> Iterator for WalkAst<'a> {
    type Item = (&'a Node, isize); // (node, depth)

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((node, depth)) = self.queue.pop() {
            if let Some(children) = node.children() {
                self.queue.extend(children.iter().map(|node| (node, depth + 1)));
            }
            return Some((node, depth))
        }
        None
    }
}

/// Iterate over the ast
pub fn iter_tree(tree: &Node) -> WalkAst<'_> {
    WalkAst{queue: vec![(tree, 0)]}
}
 
pub fn iter_tree_with_stop<'a, Predicate>(tree: &'a Node, predicate: Predicate) -> StopWalkAst<'a, Predicate>
where Predicate: Fn(&Node) -> bool + 'static
{
    StopWalkAst{queue: vec![(tree, 0)], predicate}
}

pub fn is_yaml(tree: &Node) -> bool {
    match tree {
        Node::Yaml(_) => true,
        _ => false
    }  
}

pub fn is_frontmatter(tree: &Node) -> bool {
    is_yaml(tree)
}

/// Check if the node is a link reference
///
/// [label]
pub fn is_link_reference(tree: &Node) -> bool {
    match tree {
        Node::LinkReference(_) => true,
        _ => false
    }    
}

pub fn iter_link_references<'a>(tree: &'a Node, max_depth: isize) -> impl Iterator<Item=&'a LinkReference> {
    iter_tree(tree)
    .limit_depth(max_depth)
    .filter(|node| is_link_reference(*node))
    .map(|node| {
        match node {
            Node::LinkReference(lnk) => lnk,
            _ => panic!("not a link reference")
        }
    })
}


/// Check if the node is a list item with a checkbox
///
/// - [ ] Task #1
pub fn is_checkable_item(tree: &Node) -> bool {
    match tree {
        Node::ListItem(item) => item.checked.is_some(),
        _ => false
    }
}

pub fn expect_list_item(node: &Node) -> &ListItem {
    match node {
        Node::ListItem(item) => &item,
        _ => panic!("not a list item node")
    }
}

pub fn find_frontmatter(tree: &Node) -> Option<&Node> {
    iter_tree(tree)
    .limit_depth(-1)
    .find(|node| is_frontmatter(*node))
}

