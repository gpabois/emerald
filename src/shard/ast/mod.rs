pub mod node;

pub use node::*;
use paste::paste;

use markdown::{mdast, to_mdast, Constructs, ParseOptions};

pub type NodeArena = generational_arena::Arena<Node>;

pub struct Ast {
    arena: NodeArena,
    root: NodeIndex
}

impl Ast {
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
        let mut arena = generational_arena::Arena::new();
        let root = convert_raw_node(&mut arena, tree);
        Self {arena, root}
    }
}

macro_rules! unpack {
    ($prop:ident) => {
        $prop: node.$prop
    };

    ($prop:ident, ($props:ident),+) => {
        $prop: node.$prop,
        unpack!(($props:ident),+)
    };
}

macro_rules! convert {
    (($arena:ident, $node:ident) { $($typ:ident => $body:expr);+}) => {
        match node {
            $(mdast::Node:: $typ ($node) => convert!($arena, $node, $typ, $body)),+
        }
    };

    ($arena:ident, $node:ident, $typ:ident, []) => {
        {
            let children = convert_raw_nodes($arena, $node.children);
            let node = paste!(Node.[<new_ $typ:snake>](children, $node.position));
            Some(arena.push(node))
        }
    };
}


macro_rules! crhs {
    ($type:ident, []) => {
        {
            let children = convert_raw_nodes(arena, node.children);
            let node = paste!(Node.[<new_ $type:snake>](children, node.position));
            Some(arena.push(node))
        }
    };
    ($type:ident, [$($props:ident),+]) => {
        paste! {
            {
                let children = convert_raw_nodes(arena, node.children);
                let node = Node::[<new_ $type:snake>](
                    $type {
                        unpack!($($props:ident),+)
                    },
                    children, 
                    node.position
                );
                arena.push(node)
            }  
        }
    };

    ($type:ident, $props:tt) => {
        paste! {
            {
                let children = convert_raw_nodes(arena, node.children);
                let node = Node::[<new_$type:snake>](
                    $type $props,
                    children, 
                    node.position
                );

                Some(arena.push(node))
            }
        }
    };
}

fn convert_raw_nodes(arena: &mut NodeArena, nodes: Vec<mdast::Node>) -> impl Iterator<Item=NodeIndex> + '_ {
    nodes.into_iter().flat_map(|node| convert_raw_node(arena, node))
}

fn convert_raw_node(arena: &mut NodeArena, node: mdast::Node) -> Option<NodeIndex> {
            //FootnodeDefinition => [identifier, label],
    return convert! {
        (arena, node) {
            Root => [];
            BlockQuote => [];
        }
    };

    match node {
        //convert!{Root => {}},
        /*
        convert!(BlockQuote => {}),
        convert!(FootnodeDefinition, identifier, label),
        convert!(FootnodeReference, identifier, label),
        convert!(MdxJsxFlowElement, name, attributes),
        convert!(MdxFlowExpression, value),
        convert!(MdxJsElement, value),
        convert!(MdxJsTextElement, name, attributes),
        convert!(MdxTextExpression, value),
        convert!(List, ordered, spread, start),
        convert!(ListItem, checked, spread),
        convert!(Yaml, (serde_yaml::from_str(&node.value).ok()?)),
        convert!(Toml, (toml::from_str(&node.value).ok()?)),
        convert!(Json, (json::from_str(&node.value).ok()?)),
        convert!(Html, value),
        convert!(ThematicBreak),
        convert!(Break),
        convert!(InlineCode, value),
        convert!(InlineMath, value),
        convert!(Text, value),
        convert!(Delete),
        convert!(Emphasis),
        convert!(Strong),
        convert!(Image, alt, url, title),
        convert!(ImageReference, alt, reference_kind, identifier, label),
        convert!(Link, url, title),
        convert!(LinkReference, reference_kind, identifier, label),
        convert!(Code, value, lang, meta),
        convert!(Math, value, meta),
        convert!(Heading, depth),
        convert!(Table, align),
        convert!(TableRow),
        convert!(TableCell),
        convert!(Paragraph)
        */
    }
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

