pub mod arena;

use paste::paste;

use markdown::{mdast, to_mdast, Constructs, ParseOptions};

/// Implement node conversion 
///
/// ```
/// fn convert(
///        strategy: impl NodeConverterStrategy, 
///        node: markdown::mdast::Node
///    ) -> Option<NodeConverterStrategy::NodeRef> {
///     convert!(strategy, node)
/// }
/// ```
macro_rules! convert {
    ($strategy:ident, $node:ident) => {
        convert! {
            ($strategy, $node) {
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
    };

    (($strategy:ident, $node:ident) {$($typ:ident => $body:tt;)+}) => {
        match $node {
            $(markdown::mdast::Node:: $typ ($node) => convert!($strategy, $node, $typ, $body)),+
        }
    };

    ($strategy:ident, $typ:ident, ()) => {
        {
            let children = $strategy.convert_children($node.children);
            let node = paste!($strategy::Node::[<new_ $typ:snake>](children, $node.position));
            Some($converter.insert(node))
        }
    };

    ($strategy:ident, $node:ident, $typ:ident, (no_children)) => {
        {
            let children = convert_raw_nodes($arena, Vec::default());
            let node = paste!($strategy::Node::[<new_ $typ:snake>](children, $node.position));
            Some($strategy.insert_node(node))
        }
    };

    ($strategy:ident, $node:ident, $typ:ident, [$($attr:ident),+]) => {
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


    ($strategy:ident, $node:ident, $typ:ident, (no_children, [$($attr:ident),+])) => {
        paste! {
            {
                let children = convert_raw_nodes($strategy, Vec::default());
                let node = Node::[<new_ $typ:snake>](
                    $typ {
                        $($attr: $node.$attr),+
                    },
                    children,
                    $node.position
                );
                Some($strategy.insert(node))
            }
        }
    };

    ($strategy:ident, $node:ident, $typ:ident, (no_children, $attr:expr)) => {
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
    convert!(arena, node)
}
