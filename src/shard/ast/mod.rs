use markdown::{mdast::{Node, LinkReference}, to_mdast, Constructs, ParseOptions};

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

pub fn iter_tree(tree: &Node) -> WalkAst<'_> {
    WalkAst{queue: vec![(tree, 0)]}
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

pub fn is_link_reference(tree: &Node) -> bool {
    match tree {
        Node::LinkReference(_) => true,
        _ => false
    }    
}


pub fn find_frontmatter(tree: &Node) -> Option<&Node> {
    iter_tree(tree)
    .limit_depth(-1)
    .find(|node| is_frontmatter(*node))
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

/// Représente une variable brute issue de l'AST 
pub struct RawVariable<'a> {
    name: &'a str,
    value: &'a str
}

/// Itére l'ensemble des variables
pub fn iter_raw_variables<'a>(tree: &'a Node, max_depth: isize) -> impl Iterator<Item=RawVariable<'a>> {
    iter_link_references(tree, max_depth)
    .filter(|lnk| lnk.label.is_some())
    .map(|lnk| lnk.label.as_ref().unwrap().split("::").collect::<Vec<&'a str>>())
    .filter(|lnk| lnk.len() == 2)
    .map(|mut lnk| RawVariable {
        name: lnk.remove(0),
        value: lnk.remove(1)
    })
}