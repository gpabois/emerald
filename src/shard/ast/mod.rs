pub mod node;

pub use node::*;
use paste::paste;

use markdown::{mdast, to_mdast, Constructs, ParseOptions};

pub type NodeArena = generational_arena::Arena<Node>;

pub struct Ast {
    arena: NodeArena,
    root: NodeIndex,
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
        let root = convert_raw_node(&mut arena, tree)?;
        Some(Self { arena, root })
    }
}

macro_rules! convert {
    (($arena:ident, $node:ident) {$($typ:ident => $body:tt;)+}) => {
        match $node {
            $(mdast::Node:: $typ ($node) => convert!($arena, $node, $typ, $body)),+
        }
    };

    ($arena:ident, $node:ident, $typ:ident, ()) => {
        {
            let children = convert_raw_nodes($arena, $node.children);
            let node = paste!(Node::[<new_ $typ:snake>](children, $node.position));
            Some($arena.insert(node))
        }
    };

    ($arena:ident, $node:ident, $typ:ident, (no_children)) => {
        {
            let children = convert_raw_nodes($arena, Vec::default());
            let node = paste!(Node::[<new_ $typ:snake>](children, $node.position));
            Some($arena.insert(node))
        }
    };



    ($arena:ident, $node:ident, $typ:ident, [$($attr:ident),+]) => {
        paste! {
            {
                let children = convert_raw_nodes($arena, $node.children);
                let node = Node::[<new_ $typ:snake>](
                    $typ {
                        $($attr: $node.$attr),+
                    },
                    children,
                    $node.position
                );
                Some($arena.insert(node))
            }
        }
    };


    ($arena:ident, $node:ident, $typ:ident, (no_children, [$($attr:ident),+])) => {
        paste! {
            {
                let children = convert_raw_nodes($arena, Vec::default());
                let node = Node::[<new_ $typ:snake>](
                    $typ {
                        $($attr: $node.$attr),+
                    },
                    children,
                    $node.position
                );
                Some($arena.insert(node))
            }
        }
    };

    ($arena:ident, $node:ident, $typ:ident, (no_children, $attr:expr)) => {
        paste! {
            {
                let children = convert_raw_nodes($arena, Vec::default());
                let node = Node::[<new_ $typ:snake>](
                    $attr,
                    children,
                    $node.position
                );
                Some($arena.insert(node))
            }
        }
    };

    ($arena:ident, $node:ident, $typ:ident, $attr:expr) => {
        paste! {
            {
                let children = convert_raw_nodes($arena, Vec::default());
                let node = Node::[<new_ $typ:snake>](
                    $attr,
                    children,
                    $node.position
                );
                Some($arena.insert(node))
            }
        }
    };


}

fn convert_raw_nodes(
    arena: &mut NodeArena,
    nodes: Vec<mdast::Node>,
) -> impl Iterator<Item = NodeIndex> + '_ {
    nodes
        .into_iter()
        .flat_map(|node| convert_raw_node(arena, node))
}

fn convert_raw_node(arena: &mut NodeArena, node: mdast::Node) -> Option<NodeIndex> {
    convert! {
        (arena, node) {
            Root => ();
            BlockQuote => ();
            Definition => (no_children, [url, title, identifier, label]);
            FootnoteDefinition => (no_children, [identifier, label]);
            FootnoteReference => (no_children, [identifier, label]);
            MdxJsxFlowElement => [name, attributes];
            MdxFlowExpression => (no_children, [value]);
            MdxTextExpression => (no_children, [value]);
            MdxJsxTextElement => [name, attributes];
            MdxjsEsm => (no_children, [value]);
            List => [ordered, spread, start];
            ListItem => [checked, spread];
            Yaml => (
                no_children,
                serde_yaml::from_str(&node.value).ok()?
            );
            Toml => (
                no_children,
                toml::from_str(&node.value).ok()?
            );
            Html => (no_children, node.value);
            ThematicBreak => (no_children);
            Break => (no_children);
            InlineCode => (no_children, [value]);
            InlineMath => (no_children, [value]);
            Text => (node.value);
            Delete => ();
            Emphasis => ();
            Strong => ();
            Image => (no_children, [alt, url, title]);
            ImageReference => (no_children, [alt, reference_kind, identifier, label]);
            Link => [url, title];
            LinkReference => [reference_kind, identifier, label];
            Code => (no_children, [value, lang, meta]);
            Math => (no_children, [value, meta]);
            Heading => [depth];
            Table => [align];
            TableRow => ();
            TableCell => ();
            Paragraph =>();
        }
    }
}
